use std::fs::metadata;
use tauri::State;

use crate::config::UiConfig;
use crate::crypto::{derive_key, EncryptionKey};
use crate::error::TasksError;
use crate::event::{EventStore, TaskEvent};
use crate::storage;
use crate::util::get_save_file_paths;

#[tauri::command]
pub fn check_exists() -> Result<bool, TasksError> {
	let save_file_paths = get_save_file_paths();

	let some_exist = save_file_paths
		.into_iter()
		.any(|(config_path, tasks_path)| {
			metadata(&config_path).is_ok() && metadata(&tasks_path).is_ok()
		});

	Ok(some_exist)
}

#[tauri::command]
pub fn unlock(
	password: &str,
	encryption_key: State<EncryptionKey>,
) -> Result<UiConfig, TasksError> {
	let config = storage::load_config()?;

	derive_key(
		password,
		&config.salt,
		&mut encryption_key.0.lock().unwrap(),
	)?;

	if !check_exists()? {
		storage::save_events(Vec::new(), &encryption_key)?;
	}
	Ok(config.ui)
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
