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
use crate::util::{find_first_existing_file, get_config_path, get_salt_paths, get_tasks_paths};

#[derive(Serialize, Deserialize, Debug)]
pub struct TasksData {
	pub version: String,
	pub events: Vec<TaskEvent>,
}

pub fn check_exists(config: &Config) -> Result<bool, TasksError> {
	Ok(get_tasks_paths(config).iter().any(|path| path.exists()))
}

fn save_data_to_files(data: &[u8], paths: Vec<PathBuf>) -> Result<(), TasksError> {
	paths
		.iter()
		.try_for_each(|path| write_buffer_to_file(path, &data))
}

pub fn save_salt(config: &Config, salt: &[u8; SALT_SIZE]) -> Result<(), TasksError> {
	save_data_to_files(salt, get_salt_paths(config))
}

pub fn create_new_salt(config: &Config) -> Result<Salt, TasksError> {
	let mut new_salt: Salt = [0u8; SALT_SIZE];
	generate_random_bytes(&mut new_salt);
	save_salt(config, &new_salt)?;
	Ok(new_salt)
}

pub fn load_salt(config: &Config) -> Result<[u8; SALT_SIZE], TasksError> {
	let salt_paths = get_salt_paths(config);
	let salt = match find_first_existing_file(&salt_paths) {
		Some(salt_path) => {
			let salt_vec = read_file_into_buffer(&salt_path)?;
			salt_vec
				.try_into()
				.map_err(|_| TasksError::UnknownError("Invalid salt size".to_string()))?
		}
		None => create_new_salt(config)?,
	};
	Ok(salt)
}

pub fn save_config(config: &Config) -> Result<(), TasksError> {
	let config_data = serde_json::to_string(&config)?;
	save_data_to_files(&config_data.into_bytes(), vec![get_config_path()])
}

pub fn load_config() -> Config {
	read_file_into_buffer(&get_config_path())
		.and_then(|config_json| {
			serde_json::from_slice::<Config>(&config_json).map_err(TasksError::from)
		})
		.unwrap_or_default()
}

fn encrypt_then_save(
	data: &[u8],
	encryption_key: &EncryptionKey,
	paths: Vec<PathBuf>,
) -> Result<(), TasksError> {
	let encrypted = encrypt(data, &encryption_key.0.lock().unwrap())?;
	save_data_to_files(&encrypted, paths)?;
	Ok(())
}

pub fn save_events(
	config: &Config,
	events: Vec<TaskEvent>,
	encryption_key: &State<EncryptionKey>,
) -> Result<(), TasksError> {
	let tasks_data = TasksData {
		version: SERIALIZATION_VERSION.to_string(),
		events,
	};
	let serialized_tasks_data = serde_json::to_string(&tasks_data)?.into_bytes();
	encrypt_then_save(
		&serialized_tasks_data,
		encryption_key,
		get_tasks_paths(config),
	)
}

pub fn save_event(
	config: &Config,
	event: TaskEvent,
	encryption_key: &State<EncryptionKey>,
	event_store: &State<EventStore>,
) -> Result<(), TasksError> {
	let mut events = event_store.events.lock().unwrap();
	events.insert(event.id, event);
	let sorted_events = hashmap_to_sorted_vec(&events);
	save_events(config, sorted_events, encryption_key)
}

fn process_event_data(
	encrypted_data: &[u8],
	encryption_key: &State<EncryptionKey>,
) -> Result<Vec<TaskEvent>, TasksError> {
	let tasks_json = decrypt(&encrypted_data, &encryption_key.0.lock().unwrap())?;
	let tasks_data: TasksData = serde_json::from_str(&tasks_json)?;
	Ok(tasks_data.events)
}

pub fn load_events(
	config: &Config,
	encryption_key: &State<EncryptionKey>,
	event_store: &State<EventStore>,
) -> Result<Vec<TaskEvent>, TasksError> {
	let mut all_events = event_store.events.lock().unwrap();

	for tasks_path in get_tasks_paths(config) {
		if let Ok(encrypted_data) = read_file_into_buffer(&tasks_path) {
			let file_events = process_event_data(&encrypted_data, encryption_key)?;

			for event in file_events {
				all_events.entry(event.id).or_insert(event);
			}
		}
	}

	let sorted_events = hashmap_to_sorted_vec(&all_events);
	Ok(sorted_events)
}

pub fn change_password(
	config: &Config,
	current_password: &str,
	new_password: &str,
	encryption_key: &State<EncryptionKey>,
	event_store: &State<EventStore>,
) -> Result<(), TasksError> {
	let current_salt = load_salt(config)?;
	let mut key_to_check = [0; ENCRYPTION_KEY_SIZE];
	derive_key(current_password, &current_salt, &mut key_to_check)?;
	if key_to_check != *encryption_key.0.lock().unwrap() {
		return Err(TasksError::CryptoError("Incorrect password".to_string()));
	}
	let events = load_events(config, encryption_key, event_store)?;

	let new_salt = create_new_salt(config)?;
	derive_key(
		new_password,
		&new_salt,
		&mut encryption_key.0.lock().unwrap(),
	)?;
	save_events(config, events, &encryption_key)?;

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::config::{SHUSHING_FACE_DIRNAME, TASKS_FILENAME};
	use std::fs::{self, File};
	use tempfile::tempdir;

	fn setup() -> (Config, PathBuf) {
		let tmp_dir = tempdir().unwrap();
		let config: Config = Default::default();
		(config, tmp_dir.into_path())
	}

	fn teardown(tmp_dir: PathBuf) {
		fs::remove_dir_all(tmp_dir).unwrap();
	}

	#[test]
	fn test_check_exists() {
		let (config, tmp_dir) = setup();
		let path = tmp_dir.join(SHUSHING_FACE_DIRNAME).join(TASKS_FILENAME);
		fs::create_dir_all(path.parent().unwrap()).unwrap();
		File::create(&path).unwrap();

		let exists = check_exists(&config).unwrap();
		assert!(exists);

		teardown(tmp_dir);
	}

	#[test]
	fn test_save_and_load_salt() {
		let (config, tmp_dir) = setup();
		let salt = create_new_salt(&config).unwrap();
		let loaded_salt = load_salt(&config).unwrap();

		assert_eq!(salt, loaded_salt);

		teardown(tmp_dir);
	}
}
