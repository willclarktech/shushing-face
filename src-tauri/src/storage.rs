use serde::{Deserialize, Serialize};
use tauri::State;

use crate::config::SERIALIZATION_VERSION;
use crate::crypto::{decrypt, derive_key, encrypt, EncryptionKey, ENCRYPTION_KEY_SIZE, SALT_SIZE};
use crate::error::TasksError;
use crate::event::TaskEvent;
use crate::fs::{read_file_into_buffer, write_buffer_to_file};
use crate::util::{find_first_existing_file, get_salt_paths, get_tasks_paths};

#[derive(Serialize, Deserialize, Debug)]
pub struct TasksData {
	pub version: String,
	pub events: Vec<TaskEvent>,
}

pub fn save_events(
	events: Vec<TaskEvent>,
	encryption_key: &State<EncryptionKey>,
) -> Result<(), TasksError> {
	let tasks_data = TasksData {
		version: SERIALIZATION_VERSION.to_string(),
		events,
	};
	let serialized_tasks_data = serde_json::to_string(&tasks_data).map_err(TasksError::from)?;
	let encrypted_data = encrypt(&serialized_tasks_data, &encryption_key.0.lock().unwrap())
		.map_err(TasksError::from)?;

	for tasks_path in get_tasks_paths() {
		write_buffer_to_file(&tasks_path, &encrypted_data)?;
	}

	Ok(())
}

pub fn save_event(
	event: TaskEvent,
	encryption_key: &State<EncryptionKey>,
) -> Result<(), TasksError> {
	let mut events = load_events(encryption_key)?;
	events.push(event);
	save_events(events, encryption_key)
}

pub fn load_events(encryption_key: &State<EncryptionKey>) -> Result<Vec<TaskEvent>, TasksError> {
	if let Some(tasks_path) = find_first_existing_file(&get_tasks_paths()) {
		let encrypted_data = read_file_into_buffer(&tasks_path)?;
		let tasks_json = decrypt(&encrypted_data, &encryption_key.0.lock().unwrap())
			.map_err(TasksError::from)?;

		let tasks_data: TasksData = serde_json::from_str(&tasks_json).map_err(TasksError::from)?;
		Ok(tasks_data.events)
	} else {
		Ok(Vec::new())
	}
}

pub fn change_password(
	current: &str,
	new: &str,
	encryption_key: &State<EncryptionKey>,
) -> Result<(), TasksError> {
	let salt_paths = get_salt_paths();

	let mut salt = [0; SALT_SIZE];
	if let Some(salt_path) = find_first_existing_file(&salt_paths) {
		let salt_data = read_file_into_buffer(&salt_path)?;
		salt.copy_from_slice(&salt_data);
	} else {
		return Err(TasksError::CryptoError("Salt file not found".to_string()));
	}

	let mut key_to_check = [0; ENCRYPTION_KEY_SIZE];
	derive_key(current, &salt, &mut key_to_check)?;
	if key_to_check != *encryption_key.0.lock().unwrap() {
		return Err(TasksError::CryptoError("Incorrect password".to_string()));
	}

	let events = load_events(&encryption_key)?;
	derive_key(new, &salt, &mut encryption_key.0.lock().unwrap())?;
	save_events(events, &encryption_key)?;

	Ok(())
}
