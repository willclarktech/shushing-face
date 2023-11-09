// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod crypto;
mod tasks;

use crate::crypto::EncryptionKey;
use crate::tasks::{load_tasks, save_tasks, unlock};

fn main() {
	tauri::Builder::default()
		.manage(EncryptionKey(Default::default()))
		.invoke_handler(tauri::generate_handler![unlock, save_tasks, load_tasks])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
