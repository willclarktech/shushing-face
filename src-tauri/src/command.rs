use std::fs::metadata;
use tauri::State;

use crate::crypto::{derive_key, generate_random_bytes, EncryptionKey, SALT_SIZE};
use crate::error::TasksError;
use crate::event::TaskEvent;
use crate::fs::{read_file_into_buffer, write_buffer_to_file};
use crate::storage;
use crate::util::{find_first_existing_file, get_salt_paths, get_save_file_paths};

#[tauri::command]
pub fn check_exists() -> Result<bool, String> {
	let save_file_paths = get_save_file_paths();

	let some_exist = save_file_paths.into_iter().any(|(salt_path, tasks_path)| {
		metadata(&salt_path).is_ok() && metadata(&tasks_path).is_ok()
	});

	Ok(some_exist)
}

#[tauri::command]
pub fn unlock(password: &str, encryption_key: State<EncryptionKey>) -> Result<(), TasksError> {
	let salt_paths = get_salt_paths();
	let exists_already = check_exists()?;

	let mut salt = [0; SALT_SIZE];
	if let Some(salt_path) = find_first_existing_file(&salt_paths) {
		let salt_data = read_file_into_buffer(&salt_path)?;
		salt.copy_from_slice(&salt_data);
		true
	} else {
		generate_random_bytes(&mut salt);
		false
	};

	for salt_path in salt_paths {
		write_buffer_to_file(&salt_path, &salt)?;
	}

	derive_key(password, &salt, &mut encryption_key.0.lock().unwrap())?;
	if !exists_already {
		storage::save_events(Vec::new(), &encryption_key)?;
	}
	Ok(())
}

#[tauri::command]
pub fn lock(encryption_key: State<EncryptionKey>) -> Result<(), TasksError> {
	for byte in encryption_key.0.lock().unwrap().iter_mut() {
		*byte = 0;
	}
	Ok(())
}

#[tauri::command]
pub fn change_password(
	current: &str,
	new: &str,
	encryption_key: State<EncryptionKey>,
) -> Result<(), TasksError> {
	storage::change_password(current, new, &encryption_key)
}

#[tauri::command]
pub fn save_event(
	event: TaskEvent,
	encryption_key: State<EncryptionKey>,
) -> Result<(), TasksError> {
	storage::save_event(event, &encryption_key)
}

#[tauri::command]
pub fn load_events(encryption_key: State<EncryptionKey>) -> Result<Vec<TaskEvent>, TasksError> {
	storage::load_events(&encryption_key)
}
