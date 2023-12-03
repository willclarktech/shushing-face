use serde::{Deserialize, Serialize};
use serde_with::{hex::Hex, serde_as};
use std::sync::Mutex;

use crate::crypto::SALT_SIZE;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiConfig {
	pub auto_lock_timeout: u32,
}

impl Default for UiConfig {
	fn default() -> Self {
		UiConfig {
			auto_lock_timeout: 10 * 60 * 1000, // 10 minutes
		}
	}
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
	#[serde_as(as = "Hex")]
	pub salt: [u8; SALT_SIZE],
	pub ui: UiConfig,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			salt: [0; SALT_SIZE],
			ui: Default::default(),
		}
	}
}

impl Config {
	pub fn new() -> Self {
		Default::default()
	}
}

pub struct AppConfig {
	pub config: Mutex<Config>,
}

impl Default for AppConfig {
	fn default() -> Self {
		AppConfig {
			config: Default::default(),
		}
	}
}

impl AppConfig {
	pub fn new() -> Self {
		Default::default()
	}
}

pub const SERIALIZATION_VERSION: &str = "1";

pub const YO_DIRNAME: &str = ".yo";
pub const CONFIG_FILENAME: &str = "config.json";
// Nested under home dir
pub const ICLOUD_DIRNAME: &str = "Library/Mobile Documents/com~apple~CloudDocs";
// Dropbox config: ~/.dropbox/info.json
// Nested under home dir
pub const DROPBOX_DIRNAME: &str = "Library/CloudStorage/Dropbox";

pub const TASKS_FILENAME: &str = "tasks";
