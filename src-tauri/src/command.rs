use tauri::State;

use crate::config::{AppConfig, Config};
use crate::crypto::{derive_key, EncryptionKey};
use crate::error::TasksError;
use crate::event::{EventStore, TaskEvent};
use crate::storage::{self, load_salt};
use crate::util::get_tasks_paths;

#[tauri::command]
pub fn check_exists() -> Result<bool, TasksError> {
	let tasks_exists = get_tasks_paths().into_iter().any(|path| path.exists());
	Ok(tasks_exists)
}

#[tauri::command]
pub fn unlock(
	password: &str,
	encryption_key: State<EncryptionKey>,
	app_config: State<AppConfig>,
) -> Result<Config, TasksError> {
	let tasks_exist = check_exists()?;
	let salt = load_salt()?;
	derive_key(password, &salt, &mut encryption_key.0.lock().unwrap())?;

	let mut config = app_config.config.lock().unwrap();
	*config = storage::load_config(&encryption_key)?;

	if !tasks_exist {
		storage::save_events(Vec::new(), &encryption_key)?;
	}
	Ok(config.clone())
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
pub fn update_config(
	ui_config: Config,
	encryption_key: State<EncryptionKey>,
	app_config: State<AppConfig>,
) -> Result<(), TasksError> {
	let mut new_config = app_config.config.lock().unwrap();
	*new_config = ui_config;
	storage::save_config(&new_config, &encryption_key)
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
