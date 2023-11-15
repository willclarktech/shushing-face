use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
	pub id: u64,
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
