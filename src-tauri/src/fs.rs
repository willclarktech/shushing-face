use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::error::TasksError;

pub fn read_file_into_buffer(path: &PathBuf) -> Result<Vec<u8>, TasksError> {
	let mut file = File::open(path).map_err(TasksError::from)?;
	let mut buffer = Vec::new();
	file.read_to_end(&mut buffer).map_err(TasksError::from)?;
	Ok(buffer)
}

pub fn write_buffer_to_file(path: &PathBuf, buffer: &[u8]) -> Result<(), TasksError> {
	if let Some(parent_dir) = path.parent() {
		create_dir_all(parent_dir).map_err(TasksError::from)?;
	}
	let mut file = File::create(path).map_err(TasksError::from)?;
	file.write_all(buffer).map_err(TasksError::from)?;
	Ok(())
}
