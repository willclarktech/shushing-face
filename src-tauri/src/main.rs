// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod config;
mod crypto;
mod error;
mod event;
mod fs;
mod storage;
mod task;
mod util;

use crate::command::{
	change_password, check_exists, load_events, lock, save_event, unlock, update_config,
};
use crate::config::AppConfig;
use crate::crypto::EncryptionKey;
use crate::event::EventStore;

fn main() {
	tauri::Builder::default()
		.manage(EncryptionKey(Default::default()))
		.manage(AppConfig::new())
		.manage(EventStore::new())
		.invoke_handler(tauri::generate_handler![
			change_password,
			check_exists,
			load_events,
			lock,
			save_event,
			unlock,
			update_config,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
