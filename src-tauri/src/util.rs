use std::path::PathBuf;

use home::home_dir;

use crate::config::{
	Config, CONFIG_FILENAME, DROPBOX_DIRNAME, ICLOUD_DIRNAME, SALT_FILENAME, TASKS_FILENAME,
	YO_DIRNAME,
};

fn get_home_dir() -> PathBuf {
	home_dir().expect("Failed to get home directory")
}

fn get_paths_for_file(config: &Config, file_name: &str) -> Vec<PathBuf> {
	let home = get_home_dir();
	let mut dirs = vec![home];

	if config.icloud_enabled {
		dirs.push(get_home_dir().join(ICLOUD_DIRNAME));
	}
	if config.dropbox_enabled {
		dirs.push(get_home_dir().join(DROPBOX_DIRNAME));
	}

	dirs.into_iter()
		.map(|dir| dir.join(YO_DIRNAME).join(file_name))
		.collect()
}

pub fn get_salt_paths(config: &Config) -> Vec<PathBuf> {
	get_paths_for_file(config, SALT_FILENAME)
}

pub fn get_config_path() -> PathBuf {
	get_home_dir().join(YO_DIRNAME).join(CONFIG_FILENAME)
}

pub fn get_tasks_paths(config: &Config) -> Vec<PathBuf> {
	get_paths_for_file(config, TASKS_FILENAME)
}

pub fn find_first_existing_file(paths: &[PathBuf]) -> Option<PathBuf> {
	paths.iter().find(|path| path.exists()).cloned()
}

// pub fn to_hex_string(bytes: &[u8]) -> String {
// 	bytes
// 		.map(|byte| format!("{:02x}", byte))
// 		.collect::<String>()
// }
