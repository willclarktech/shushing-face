use std::path::PathBuf;

use home::home_dir;

use crate::config::{
	Config, CONFIG_FILENAME, DROPBOX_DIRNAME, ICLOUD_DIRNAME, SALT_FILENAME, SHUSHING_FACE_DIRNAME,
	TASKS_FILENAME,
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
		.map(|dir| dir.join(SHUSHING_FACE_DIRNAME).join(file_name))
		.collect()
}

pub fn get_salt_paths(config: &Config) -> Vec<PathBuf> {
	get_paths_for_file(config, SALT_FILENAME)
}

pub fn get_config_path() -> PathBuf {
	get_home_dir()
		.join(SHUSHING_FACE_DIRNAME)
		.join(CONFIG_FILENAME)
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

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs::File;
	use tempfile::tempdir;

	#[test]
	fn test_get_paths_for_file() {
		let config = Config {
			auto_lock_timeout: 10,
			icloud_enabled: false,
			dropbox_enabled: false,
		};

		let paths = get_paths_for_file(&config, "test_file.txt");
		assert!(!paths.is_empty(), "Paths should not be empty.");
	}

	#[test]
	fn test_find_first_existing_file() {
		let dir = tempdir().unwrap();
		let file_path1 = dir.path().join("test_file1.txt");
		let file_path2 = dir.path().join("test_file2.txt");

		File::create(&file_path2).unwrap(); // Create only the second file

		let paths = vec![file_path1, file_path2.clone()];
		let existing_file = find_first_existing_file(&paths);

		assert_eq!(
			existing_file,
			Some(file_path2),
			"Should find the first existing file."
		);
	}
}
