use serde::{Deserialize, Serialize};

use crate::task::{Task, TaskEdit, TaskId};

pub type EventId = u64;

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskEventData {
	CreateTask(Task),
	CompleteTask(TaskId),
	UncompleteTask(TaskId),
	EditTask(TaskId, TaskEdit),
	DeleteTask(TaskId),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskEvent {
	pub id: EventId,
	pub data: TaskEventData,
}
