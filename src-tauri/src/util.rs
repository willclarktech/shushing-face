use std::path::PathBuf;

use home::home_dir;

use crate::config::{
	CONFIG_FILENAME, DROPBOX_DIRNAME, ICLOUD_DIRNAME, SALT_FILENAME, TASKS_FILENAME, YO_DIRNAME,
};

pub fn get_paths_for_file(file_name: &str) -> Vec<PathBuf> {
	let home = home_dir().expect("Failed to get home directory");
	let icloud = home.join(ICLOUD_DIRNAME);
	let dropbox = home.join(DROPBOX_DIRNAME);
	let dirs = vec![home, icloud, dropbox];

	dirs.into_iter()
		.map(|dir| dir.join(YO_DIRNAME).join(file_name))
		.collect()
}

pub fn get_salt_paths() -> Vec<PathBuf> {
	get_paths_for_file(SALT_FILENAME)
}

pub fn get_config_paths() -> Vec<PathBuf> {
	get_paths_for_file(CONFIG_FILENAME)
}

pub fn get_tasks_paths() -> Vec<PathBuf> {
	get_paths_for_file(TASKS_FILENAME)
}

pub fn find_first_existing_file(paths: &[PathBuf]) -> Option<PathBuf> {
	paths.iter().find(|path| path.exists()).cloned()
}

// pub fn to_hex_string(bytes: &[u8]) -> String {
// 	bytes
// 		.map(|byte| format!("{:02x}", byte))
// 		.collect::<String>()
// }
