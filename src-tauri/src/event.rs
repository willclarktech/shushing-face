use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

use crate::task::{Task, TaskEdit, TaskId};

pub type EventId = u64;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskEventData {
	CreateTask(Task),
	CompleteTask(TaskId),
	UncompleteTask(TaskId),
	EditTask(TaskId, TaskEdit),
	DeleteTask(TaskId),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskEvent {
	pub id: EventId,
	pub data: TaskEventData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventStore {
	pub events: Mutex<HashMap<EventId, TaskEvent>>,
}

impl EventStore {
	pub fn new() -> Self {
		EventStore {
			events: Mutex::new(HashMap::new()),
		}
	}
}

pub fn hashmap_to_sorted_vec(hashmap: &HashMap<EventId, TaskEvent>) -> Vec<TaskEvent> {
	let mut events: Vec<TaskEvent> = hashmap.values().cloned().collect();
	events.sort_by_key(|event| event.id);
	events
}
