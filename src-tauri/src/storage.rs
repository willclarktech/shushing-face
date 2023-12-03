use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::config::{Config, SERIALIZATION_VERSION};
use crate::crypto::{
	decrypt, derive_key, encrypt, generate_random_bytes, EncryptionKey, Salt, ENCRYPTION_KEY_SIZE,
	SALT_SIZE,
};
use crate::error::TasksError;
use crate::event::{hashmap_to_sorted_vec, EventStore, TaskEvent};
use crate::fs::{read_file_into_buffer, write_buffer_to_file};
use crate::util::{find_first_existing_file, get_config_paths, get_salt_paths, get_tasks_paths};

#[derive(Serialize, Deserialize, Debug)]
pub struct TasksData {
	pub version: String,
	pub events: Vec<TaskEvent>,
}

pub fn save_salt(salt: &[u8; SALT_SIZE]) -> Result<(), TasksError> {
	for salt_path in get_salt_paths() {
		write_buffer_to_file(&salt_path, salt)?;
	}
	Ok(())
}

pub fn create_new_salt() -> Result<Salt, TasksError> {
	let mut new_salt: Salt = [0u8; SALT_SIZE];
	generate_random_bytes(&mut new_salt);
	save_salt(&new_salt)?;
	Ok(new_salt)
}

pub fn load_salt() -> Result<[u8; SALT_SIZE], TasksError> {
	let salt_paths = get_salt_paths();
	let salt = match find_first_existing_file(&salt_paths) {
		Some(salt_path) => {
			let salt_vec = read_file_into_buffer(&salt_path)?;
			salt_vec
				.try_into()
				.map_err(|_| TasksError::UnknownError("Invalid salt size".to_string()))?
		}
		None => create_new_salt()?,
	};
	Ok(salt)
}

pub fn encrypt_then_save(
	data: &String,
	encryption_key: &EncryptionKey,
	paths: Vec<PathBuf>,
) -> Result<(), TasksError> {
	let encrypted = encrypt(data, &encryption_key.0.lock().unwrap())?;
	for path in paths {
		write_buffer_to_file(&path, &encrypted)?;
	}
	Ok(())
}

pub fn save_config(
	config: &Config,
	encryption_key: &State<EncryptionKey>,
) -> Result<(), TasksError> {
	let config_data = serde_json::to_string(&config)?;
	encrypt_then_save(&config_data, encryption_key, get_config_paths())
}

pub fn load_config(encryption_key: &State<EncryptionKey>) -> Result<Config, TasksError> {
	let config_paths = get_config_paths();
	let config = match find_first_existing_file(&config_paths) {
		Some(config_path) => {
			let encrypted_data = read_file_into_buffer(&config_path)?;
			let config_json = decrypt(&encrypted_data, &encryption_key.0.lock().unwrap())?;
			serde_json::from_str(&config_json)?
		}
		None => Config::default(),
	};
	Ok(config)
}

pub fn save_events(
	events: Vec<TaskEvent>,
	encryption_key: &State<EncryptionKey>,
) -> Result<(), TasksError> {
	let tasks_data = TasksData {
		version: SERIALIZATION_VERSION.to_string(),
		events,
	};
	let serialized_tasks_data = serde_json::to_string(&tasks_data)?;
	encrypt_then_save(&serialized_tasks_data, encryption_key, get_tasks_paths())
}

pub fn save_event(
	event: TaskEvent,
	encryption_key: &State<EncryptionKey>,
	event_store: &State<EventStore>,
) -> Result<(), TasksError> {
	let mut events = event_store.events.lock().unwrap();
	events.insert(event.id, event);
	let sorted_events = hashmap_to_sorted_vec(&events);
	save_events(sorted_events, encryption_key)
}

fn process_event_data(
	encrypted_data: Vec<u8>,
	encryption_key: &State<EncryptionKey>,
) -> Result<Vec<TaskEvent>, TasksError> {
	let tasks_json = decrypt(&encrypted_data, &encryption_key.0.lock().unwrap())?;
	let tasks_data: TasksData = serde_json::from_str(&tasks_json)?;

	Ok(tasks_data.events)
}

pub fn load_events(
	encryption_key: &State<EncryptionKey>,
	event_store: &State<EventStore>,
) -> Result<Vec<TaskEvent>, TasksError> {
	let mut all_events = event_store.events.lock().unwrap();

	for tasks_path in get_tasks_paths() {
		if let Ok(encrypted_data) = read_file_into_buffer(&tasks_path) {
			let file_events = process_event_data(encrypted_data, encryption_key)?;

			for event in file_events {
				all_events.entry(event.id).or_insert(event);
			}
		}
	}

	let sorted_events = hashmap_to_sorted_vec(&all_events);
	Ok(sorted_events)
}

pub fn change_password(
	current: &str,
	new: &str,
	encryption_key: &State<EncryptionKey>,
	event_store: &State<EventStore>,
) -> Result<(), TasksError> {
	let old_salt = load_salt()?;
	let mut key_to_check = [0; ENCRYPTION_KEY_SIZE];
	derive_key(current, &old_salt, &mut key_to_check)?;
	if key_to_check != *encryption_key.0.lock().unwrap() {
		return Err(TasksError::CryptoError("Incorrect password".to_string()));
	}

	let events = load_events(encryption_key, event_store)?;
	let new_salt = create_new_salt()?;
	derive_key(new, &new_salt, &mut encryption_key.0.lock().unwrap())?;
	save_events(events, &encryption_key)?;

	Ok(())
}
