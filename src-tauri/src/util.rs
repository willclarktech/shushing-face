use std::path::PathBuf;

use home::home_dir;

use crate::config::{CONFIG_FILENAME, DROPBOX_DIRNAME, ICLOUD_DIRNAME, TASKS_FILENAME, YO_DIRNAME};

pub fn get_save_file_paths() -> Vec<(PathBuf, PathBuf)> {
	let home = home_dir().expect("Failed to get home directory");
	let icloud = home.join(ICLOUD_DIRNAME);
	let dropbox = home.join(DROPBOX_DIRNAME);
	let dirs = vec![home, icloud, dropbox];
	let mut paths = Vec::new();

	for dir in dirs {
		let config_path = dir.join(YO_DIRNAME).join(CONFIG_FILENAME);
		let tasks_path = dir.join(YO_DIRNAME).join(TASKS_FILENAME);
		paths.push((config_path, tasks_path));
	}

	paths
}

pub fn get_tasks_paths() -> Vec<PathBuf> {
	get_save_file_paths()
		.into_iter()
		.map(|(_, tasks_path)| tasks_path)
		.collect()
}

pub fn get_config_paths() -> Vec<PathBuf> {
	get_save_file_paths()
		.into_iter()
		.map(|(config_path, _)| config_path)
		.collect()
}

pub fn find_first_existing_file(paths: &[PathBuf]) -> Option<PathBuf> {
	paths.iter().find(|path| path.exists()).cloned()
}

// pub fn to_hex_string(bytes: &[u8]) -> String {
// 	bytes
// 		.map(|byte| format!("{:02x}", byte))
// 		.collect::<String>()
// }
