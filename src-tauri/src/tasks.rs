use serde::{Deserialize, Serialize};
use std::fs::{create_dir, metadata, File};
use std::io::{Read, Write};
use tauri::State;

use crate::crypto::{
	decrypt, derive_key, encrypt, generate_random_bytes, EncryptionKey, ENCRYPTION_KEY_SIZE,
	SALT_SIZE,
};
use crate::error::TasksError;
use crate::util::{get_salt_path, get_tasks_path};

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
	let salt_path = get_salt_path();
	let tasks_path = get_tasks_path();
	Ok(metadata(salt_path).is_ok() && metadata(tasks_path).is_ok())
}

#[tauri::command]
pub fn unlock(password: &str, encryption_key: State<EncryptionKey>) -> Result<(), TasksError> {
	let salt_path = get_salt_path();
	let mut salt = [0; SALT_SIZE];
	let exists_already = check_exists()?;
	match File::open(salt_path) {
		Ok(mut file) => {
			file.read(&mut salt).map_err(TasksError::from)?;
		}
		Err(e) => {
			println!("ERR {}", e.to_string());
			generate_random_bytes(&mut salt);

			let salt_path = get_salt_path();
			if let Some(parent) = salt_path.parent() {
				// TODO: Handle nested directories in custom config
				create_dir(parent).map_err(TasksError::from)?;
			}
			let mut file = File::create(salt_path).map_err(TasksError::from)?;
			file.write_all(&salt).map_err(TasksError::from)?;
		}
	}
	derive_key(password, &salt, &mut encryption_key.0.lock().unwrap())?;
	if !exists_already {
		_save_tasks(Vec::new(), &encryption_key)?
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
	let mut salt = [0; SALT_SIZE];
	let salt_path = get_salt_path();
	let mut salt_file = File::open(salt_path)?;
	salt_file.read(&mut salt).map_err(TasksError::from)?;

	let mut key_to_check = [0; ENCRYPTION_KEY_SIZE];
	derive_key(current, &salt, &mut key_to_check)?;
	if key_to_check != *encryption_key.0.lock().unwrap() {
		return Err(TasksError::CryptoError("Incorrect password".to_string()));
	}

	let tasks = _load_tasks(&encryption_key)?;
	derive_key(new, &salt, &mut encryption_key.0.lock().unwrap())?;
	_save_tasks(tasks, encryption_key)?;

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

	let tasks_path = get_tasks_path();
	let mut file = File::create(tasks_path).map_err(TasksError::from)?;
	file.write_all(&encrypted_data).map_err(TasksError::from)?;

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
	let tasks_path = get_tasks_path();
	match File::open(tasks_path) {
		Ok(mut file) => {
			let mut encrypted_data = Vec::new();
			file.read_to_end(&mut encrypted_data)
				.map_err(TasksError::from)?;

			let tasks_json = decrypt(&encrypted_data, &encryption_key.0.lock().unwrap())
				.map_err(TasksError::from)?;

			serde_json::from_str(&tasks_json).map_err(TasksError::from)
		}
		Err(_) => Ok(Default::default()),
	}
}

#[tauri::command]
pub fn load_tasks(encryption_key: State<EncryptionKey>) -> Result<Vec<Task>, TasksError> {
	_load_tasks(&encryption_key)
}
