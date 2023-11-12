// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod crypto;
mod error;
mod tasks;
mod util;

use crate::crypto::EncryptionKey;
use crate::tasks::{change_password, check_exists, load_tasks, lock, save_tasks, unlock};

fn main() {
	tauri::Builder::default()
		.manage(EncryptionKey(Default::default()))
		.invoke_handler(tauri::generate_handler![
			change_password,
			check_exists,
			load_tasks,
			lock,
			save_tasks,
			unlock,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
