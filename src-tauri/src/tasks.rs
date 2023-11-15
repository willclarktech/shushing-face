use serde::{Deserialize, Serialize};
use std::fs::metadata;
use tauri::State;

use crate::crypto::{
	decrypt, derive_key, encrypt, generate_random_bytes, EncryptionKey, ENCRYPTION_KEY_SIZE,
	SALT_SIZE,
};
use crate::error::TasksError;
use crate::fs::{read_file_into_buffer, write_buffer_to_file};
use crate::util::{find_first_existing_file, get_salt_paths, get_save_file_paths, get_tasks_paths};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
	pub id: u64,
	pub description: String,
	pub details: String,
	pub deadline: u64,
	pub completed: bool,
}

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
		_save_tasks(Vec::new(), &encryption_key)?;
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

fn _change_password(
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

	let tasks = _load_tasks(&encryption_key)?;
	derive_key(new, &salt, &mut encryption_key.0.lock().unwrap())?;
	_save_tasks(tasks, &encryption_key)?;

	Ok(())
}

#[tauri::command]
pub fn change_password(
	current: &str,
	new: &str,
	encryption_key: State<EncryptionKey>,
) -> Result<(), TasksError> {
	_change_password(current, new, &encryption_key)
}

fn _save_tasks(tasks: Vec<Task>, encryption_key: &State<EncryptionKey>) -> Result<(), TasksError> {
	let serialized_tasks = serde_json::to_string(&tasks).map_err(TasksError::from)?;
	let encrypted_data =
		encrypt(&serialized_tasks, &encryption_key.0.lock().unwrap()).map_err(TasksError::from)?;

	for tasks_path in get_tasks_paths() {
		write_buffer_to_file(&tasks_path, &encrypted_data)?;
	}

	Ok(())
}

#[tauri::command]
pub fn save_tasks(
	tasks: Vec<Task>,
	encryption_key: State<EncryptionKey>,
) -> Result<(), TasksError> {
	_save_tasks(tasks, &encryption_key)
}

fn _load_tasks(encryption_key: &State<EncryptionKey>) -> Result<Vec<Task>, TasksError> {
	if let Some(tasks_path) = find_first_existing_file(&get_tasks_paths()) {
		let encrypted_data = read_file_into_buffer(&tasks_path)?;
		let tasks_json = decrypt(&encrypted_data, &encryption_key.0.lock().unwrap())
			.map_err(TasksError::from)?;
		serde_json::from_str(&tasks_json).map_err(TasksError::from)
	} else {
		Ok(Vec::new())
	}
}

#[tauri::command]
pub fn load_tasks(encryption_key: State<EncryptionKey>) -> Result<Vec<Task>, TasksError> {
	_load_tasks(&encryption_key)
}
