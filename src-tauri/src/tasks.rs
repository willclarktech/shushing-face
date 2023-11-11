use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use tauri::State;

use crate::crypto::{
	decrypt, derive_key, encrypt, generate_random_bytes, EncryptionKey, SALT_SIZE,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
	pub id: u64,
	pub description: String,
	pub deadline: u64,
	pub completed: bool,
}

const TASKS_PATH: &str = "/Users/will/code/yo-tasks/Dropbox/.yo/tasks";
const SALT_PATH: &str = "/Users/will/code/yo-tasks/Dropbox/.yo/tasks.salt";

#[tauri::command]
pub fn unlock(password: &str, encryption_key: State<EncryptionKey>) -> Result<(), String> {
	let mut salt = [0; SALT_SIZE];
	match File::open(SALT_PATH) {
		Ok(mut file) => {
			file.read(&mut salt).map_err(|e| e.to_string())?;
		}
		Err(_) => {
			generate_random_bytes(&mut salt);
			let mut file = File::create(SALT_PATH).map_err(|e| e.to_string())?;
			file.write_all(&salt).map_err(|e| e.to_string())?;
		}
	}
	derive_key(password, &salt, &mut encryption_key.0.lock().unwrap())?;
	Ok(())
}

#[tauri::command]
pub fn lock(encryption_key: State<EncryptionKey>) -> Result<(), String> {
	for byte in encryption_key.0.lock().unwrap().iter_mut() {
		*byte = 0;
	}
	Ok(())
}

#[tauri::command]
pub fn save_tasks(tasks: Vec<Task>, encryption_key: State<EncryptionKey>) -> Result<(), String> {
	let serialized_tasks = serde_json::to_string(&tasks).map_err(|e| e.to_string())?;
	let encrypted_data =
		encrypt(&serialized_tasks, &encryption_key.0.lock().unwrap()).map_err(|e| e.to_string())?;

	let mut file = File::create(TASKS_PATH).map_err(|e| e.to_string())?;
	file.write_all(&encrypted_data).map_err(|e| e.to_string())?;

	Ok(())
}

#[tauri::command]
pub fn load_tasks(encryption_key: State<EncryptionKey>) -> Result<Vec<Task>, String> {
	match File::open(TASKS_PATH) {
		Ok(mut file) => {
			let mut encrypted_data = Vec::new();
			file.read_to_end(&mut encrypted_data)
				.map_err(|e| e.to_string())?;

			let tasks_json = decrypt(&encrypted_data, &encryption_key.0.lock().unwrap())
				.map_err(|e| e.to_string())?;

			serde_json::from_str(&tasks_json).map_err(|e| e.to_string())
		}
		Err(_) => Ok(Default::default()),
	}
}
