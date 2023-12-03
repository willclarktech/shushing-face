use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
	pub auto_lock_timeout: u32,
	pub icloud_enabled: bool,
	pub dropbox_enabled: bool,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			auto_lock_timeout: 10 * 60 * 1000, // 10 minutes
			icloud_enabled: false,
			dropbox_enabled: false,
		}
	}
}

#[derive(Default)]
pub struct AppConfig {
	pub config: Mutex<Config>,
}

impl AppConfig {
	pub fn new() -> Self {
		Default::default()
	}
}

pub const SERIALIZATION_VERSION: &str = "1";

pub const YO_DIRNAME: &str = ".yo";
pub const SALT_FILENAME: &str = "salt";
pub const CONFIG_FILENAME: &str = "config.json";
// Nested under home dir
pub const ICLOUD_DIRNAME: &str = "Library/Mobile Documents/com~apple~CloudDocs";
// Dropbox config: ~/.dropbox/info.json
// Nested under home dir
pub const DROPBOX_DIRNAME: &str = "Library/CloudStorage/Dropbox";

pub const TASKS_FILENAME: &str = "tasks";
