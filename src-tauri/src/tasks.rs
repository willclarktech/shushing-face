use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use tauri::State;

use crate::crypto::{decrypt, derive_key, encrypt, EncryptionKey};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
	pub id: u64,
	pub description: String,
	pub deadline: u64,
	pub completed: bool,
}

const TASKS_PATH: &str = "/Users/will/code/yo-tasks/Dropbox/.yo/tasks.json";
// TODO: Change this
const SALT: &[u8] = b"some fixed salt value";

#[tauri::command]
pub fn unlock(password: &str, encryption_key: State<EncryptionKey>) -> Result<(), String> {
	derive_key(password, SALT, &mut encryption_key.0.lock().unwrap())?;
	Ok(())
}

#[tauri::command]
pub fn save_tasks(tasks: Vec<Task>, encryption_key: State<EncryptionKey>) -> Result<(), String> {
	println!(
		"save encryption_key: {}",
		encryption_key
			.0
			.lock()
			.unwrap()
			.iter()
			.map(|byte| format!("{:02x}", byte))
			.collect::<String>()
	);
	let serialized_tasks = serde_json::to_string(&tasks).map_err(|e| e.to_string())?;
	let encrypted_data =
		encrypt(&serialized_tasks, &encryption_key.0.lock().unwrap()).map_err(|e| e.to_string())?;

	let mut file = File::create(TASKS_PATH).map_err(|e| e.to_string())?;
	file.write_all(&encrypted_data).map_err(|e| e.to_string())?;

	Ok(())
}

#[tauri::command]
pub fn load_tasks(encryption_key: State<EncryptionKey>) -> Result<Vec<Task>, String> {
	println!(
		"load encryption_key: {}",
		encryption_key
			.0
			.lock()
			.unwrap()
			.iter()
			.map(|byte| format!("{:02x}", byte))
			.collect::<String>()
	);
	let result = File::open(TASKS_PATH).map_err(|e| e.to_string());
	match result {
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
