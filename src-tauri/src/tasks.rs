use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

use crate::crypto::{decrypt, encrypt};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
	pub id: u64,
	pub description: String,
	pub deadline: u64,
	pub completed: bool,
}

const TASKS_PATH: &str = "/Users/will/code/yo-tasks/Dropbox/.yo/tasks.json";
// TODO: Change this
const ENCRYPTION_KEY: &[u8; 32] = b"your-32-byte-long-encryption-key";

#[tauri::command]
pub fn save_tasks(tasks: Vec<Task>) -> Result<(), String> {
	let serialized_tasks = serde_json::to_string(&tasks)?;
	let encrypted_data = encrypt(&serialized_tasks, ENCRYPTION_KEY)?;

	let mut file = File::create(TASKS_PATH)?;
	file.write_all(&encrypted_data)?;

	Ok(())
}

#[tauri::command]
pub fn load_tasks() -> Result<Vec<Task>, String> {
	let mut file = File::open(TASKS_PATH)?;

	let mut encrypted_data = Vec::new();
	file.read_to_end(&mut encrypted_data)?;

	let tasks_json = decrypt(&encrypted_data, ENCRYPTION_KEY)?;
	Ok(serde_json::from_str(&tasks_json)?)
}
