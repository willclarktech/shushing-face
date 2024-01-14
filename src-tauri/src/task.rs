use serde::{Deserialize, Serialize};

pub type TaskId = u64;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
	pub id: TaskId,
	pub description: String,
	pub deadline: u64,
	pub details: String,
	pub completed: bool,
}
