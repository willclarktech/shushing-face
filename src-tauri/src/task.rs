use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub type TaskId = u64;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Task {
	pub id: TaskId,
	pub description: String,
	pub deadline: NaiveDate,
	pub details: String,
	pub completed: bool,
}
