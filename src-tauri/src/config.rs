use serde::{Deserialize, Serialize};

use crate::crypto::SALT_SIZE;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
	pub salt: [u8; SALT_SIZE],
}

impl Default for Config {
	fn default() -> Self {
		Config {
			salt: [0; SALT_SIZE],
		}
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
pub const SALT_FILENAME: &str = "tasks.salt";
