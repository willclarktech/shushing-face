use std::path::PathBuf;

use home::home_dir;

use crate::config::{SALT_FILENAME, TASKS_FILENAME, YO_DIRNAME};

pub fn get_tasks_path() -> PathBuf {
	let home = home_dir().expect("Failed to get home directory");
	home.join(YO_DIRNAME).join(TASKS_FILENAME)
}

pub fn get_salt_path() -> PathBuf {
	let home = home_dir().expect("Failed to get home directory");
	home.join(YO_DIRNAME).join(SALT_FILENAME)
}

// pub fn to_hex_string(bytes: &[u8]) -> String {
// 	bytes
// 		.map(|byte| format!("{:02x}", byte))
// 		.collect::<String>()
// }
