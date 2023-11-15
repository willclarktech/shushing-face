use serde::{Deserialize, Serialize};

use crate::task::{Task, TaskEdit};

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskEvent {
	CreateTask(Task),
	CompleteTask(u64),
	UncompleteTask(u64),
	EditTask(u64, TaskEdit),
	DeleteTask(u64),
}
