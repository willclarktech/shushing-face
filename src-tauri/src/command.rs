use std::fs::metadata;
use tauri::State;

use crate::config::Config;
use crate::crypto::{derive_key, generate_random_bytes, EncryptionKey};
use crate::error::TasksError;
use crate::event::{EventStore, TaskEvent};
use crate::fs::read_file_into_buffer;
use crate::storage::{self, save_config};
use crate::util::{find_first_existing_file, get_config_paths, get_save_file_paths};

#[tauri::command]
pub fn check_exists() -> Result<bool, String> {
	let save_file_paths = get_save_file_paths();

	let some_exist = save_file_paths
		.into_iter()
		.any(|(config_path, tasks_path)| {
			metadata(&config_path).is_ok() && metadata(&tasks_path).is_ok()
		});

	Ok(some_exist)
}

#[tauri::command]
pub fn unlock(password: &str, encryption_key: State<EncryptionKey>) -> Result<(), TasksError> {
	let config_paths = get_config_paths();

	let config = match find_first_existing_file(&config_paths) {
		Some(config_path) => {
			let config_data = read_file_into_buffer(&config_path)?;
			serde_json::from_slice::<Config>(&config_data)?
		}
		None => {
			let mut new_config = Config::default();
			generate_random_bytes(&mut new_config.salt);
			save_config(&new_config)?;
			new_config
		}
	};

	derive_key(
		password,
		&config.salt,
		&mut encryption_key.0.lock().unwrap(),
	)?;

	if !check_exists()? {
		storage::save_events(Vec::new(), &encryption_key)?;
	}
	Ok(())
}

#[tauri::command]
pub fn lock(
	encryption_key: State<EncryptionKey>,
	event_store: State<EventStore>,
) -> Result<(), TasksError> {
	for byte in encryption_key.0.lock().unwrap().iter_mut() {
		*byte = 0;
	}
	let mut events = event_store.events.lock().unwrap();
	events.clear();
	Ok(())
}

#[tauri::command]
pub fn change_password(
	current: &str,
	new: &str,
	encryption_key: State<EncryptionKey>,
	event_store: State<EventStore>,
) -> Result<(), TasksError> {
	storage::change_password(current, new, &encryption_key, &event_store)
}

#[tauri::command]
pub fn save_event(
	event: TaskEvent,
	encryption_key: State<EncryptionKey>,
	event_store: State<EventStore>,
) -> Result<(), TasksError> {
	storage::save_event(event, &encryption_key, &event_store)
}

#[tauri::command]
pub fn load_events(
	encryption_key: State<EncryptionKey>,
	event_store: State<EventStore>,
) -> Result<Vec<TaskEvent>, TasksError> {
	storage::load_events(&encryption_key, &event_store)
}
