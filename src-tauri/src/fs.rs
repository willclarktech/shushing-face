use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::error::TasksError;

pub fn read_file_into_buffer(path: &PathBuf) -> Result<Vec<u8>, TasksError> {
	let mut file = File::open(path)?;
	let mut buffer = Vec::new();
	file.read_to_end(&mut buffer)?;
	Ok(buffer)
}

pub fn write_buffer_to_file(path: &PathBuf, buffer: &[u8]) -> Result<(), TasksError> {
	if let Some(parent_dir) = path.parent() {
		create_dir_all(parent_dir)?;
	}
	let mut file = File::create(path)?;
	file.write_all(buffer)?;
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;
	use std::io;
	use tempfile::tempdir;

	#[test]
	fn test_write_and_read_file() -> io::Result<()> {
		let dir = tempdir()?;
		let file_path = dir.path().join("test_file.txt");
		let data = b"Hello, world!";
		write_buffer_to_file(&file_path, data).expect("Failed to write to file");
		let read_data = read_file_into_buffer(&file_path).expect("Failed to read from file");
		assert_eq!(data, read_data.as_slice());
		dir.close()?;
		Ok(())
	}

	#[test]
	fn test_file_not_found() {
		let path = PathBuf::from("non_existent_file.txt");
		let result = read_file_into_buffer(&path);
		assert!(result.is_err());
	}

	#[test]
	fn test_write_to_read_only_directory() -> io::Result<()> {
		let dir = tempdir()?;
		let metadata = fs::metadata(dir.path())?;
		let mut permissions = metadata.permissions();
		permissions.set_readonly(true);
		fs::set_permissions(dir.path(), permissions)?;

		let file_path = dir.path().join("test_file.txt");
		let data = b"Hello, world!";
		let result = write_buffer_to_file(&file_path, data);
		assert!(result.is_err());
		dir.close()?;
		Ok(())
	}
}
