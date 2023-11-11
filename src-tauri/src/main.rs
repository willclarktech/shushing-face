// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod crypto;
mod error;
mod tasks;

use crate::crypto::EncryptionKey;
use crate::tasks::{check_exists, load_tasks, lock, save_tasks, unlock};

fn main() {
	tauri::Builder::default()
		.manage(EncryptionKey(Default::default()))
		.invoke_handler(tauri::generate_handler![
			check_exists,
			load_tasks,
			lock,
			save_tasks,
			unlock,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
