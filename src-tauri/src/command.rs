use tauri::State;

use crate::config::{AppConfig, Config};
use crate::crypto::{derive_key, EncryptionKey};
use crate::error::TasksError;
use crate::event::{hashmap_to_sorted_vec, EventStore, TaskEvent};
use crate::storage::{self, load_salt};

#[tauri::command]
pub fn check_exists(app_config: State<AppConfig>) -> Result<bool, TasksError> {
	let config = app_config.config.lock().unwrap();
	storage::check_exists(&config)
}

#[tauri::command]
pub fn unlock(
	password: &str,
	encryption_key: State<EncryptionKey>,
	app_config: State<AppConfig>,
) -> Result<Config, TasksError> {
	let mut config = app_config.config.lock().unwrap();
	*config = storage::load_config();

	let salt = load_salt(&config)?;
	derive_key(password, &salt, &mut encryption_key.0.lock().unwrap())?;

	if !storage::check_exists(&config)? {
		storage::save_events(&config, Vec::new(), &encryption_key)?;
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
	app_config: State<AppConfig>,
	encryption_key: State<EncryptionKey>,
	event_store: State<EventStore>,
) -> Result<(), TasksError> {
	let config = app_config.config.lock().unwrap();
	storage::change_password(&config, current, new, &encryption_key, &event_store)
}

#[tauri::command]
pub fn update_config(
	new_config: Config,
	app_config: State<AppConfig>,
	encryption_key: State<EncryptionKey>,
	event_store: State<EventStore>,
) -> Result<(), TasksError> {
	let mut config = app_config.config.lock().unwrap();
	*config = new_config;
	storage::save_config(&config)?;

	let salt = storage::load_salt(&config)?;
	storage::save_salt(&config, &salt)?;
	let events = event_store.events.lock().unwrap();
	storage::save_events(&config, hashmap_to_sorted_vec(&events), &encryption_key)
}

#[tauri::command]
pub fn save_event(
	event: TaskEvent,
	app_config: State<AppConfig>,
	encryption_key: State<EncryptionKey>,
	event_store: State<EventStore>,
) -> Result<(), TasksError> {
	let config = app_config.config.lock().unwrap();
	storage::save_event(&config, event, &encryption_key, &event_store)
}

#[tauri::command]
pub fn load_events(
	app_config: State<AppConfig>,
	encryption_key: State<EncryptionKey>,
	event_store: State<EventStore>,
) -> Result<Vec<TaskEvent>, TasksError> {
	let config = app_config.config.lock().unwrap();
	storage::load_events(&config, &encryption_key, &event_store)
}
