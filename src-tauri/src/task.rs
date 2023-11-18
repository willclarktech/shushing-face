use serde::{Deserialize, Serialize};

pub type TaskId = u64;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
	pub id: TaskId,
	pub description: String,
	pub deadline: u64,
	pub details: String,
	pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskEdit {
	pub description: String,
	pub deadline: u64,
	pub details: String,
}
