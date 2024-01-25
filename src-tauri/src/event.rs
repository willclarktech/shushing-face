use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

use crate::task::{Task, TaskId};

pub type EventId = u64;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskEventData {
	CreateTask(Task),
	UpdateTask(Task),
	DeleteTask(TaskId),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
	events.sort();
	events
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_event_store_new() {
		let store = EventStore::new();
		assert!(store.events.lock().unwrap().is_empty());
	}

	#[test]
	fn test_event_store_add_and_retrieve_event() {
		let store = EventStore::new();
		let task = Task {
			id: 1,
			description: "Test Task".to_string(),
			deadline: Default::default(),
			details: Default::default(),
			completed: false,
		};

		let event = TaskEvent {
			id: 1,
			data: TaskEventData::CreateTask(task),
		};

		{
			let mut events = store.events.lock().unwrap();
			events.insert(event.id, event.clone());
		}

		let retrieved_events = store.events.lock().unwrap();
		let retrieved_event = retrieved_events.get(&event.id);
		assert!(retrieved_event.is_some());
		assert_eq!(&event, retrieved_event.unwrap());
	}

	#[test]
	fn test_hashmap_to_sorted_vec() {
		let mut hashmap = HashMap::new();
		let task1 = Task {
			id: 1,
			description: "Task 1".to_string(),
			deadline: Default::default(),
			details: Default::default(),
			completed: false,
		};
		let task2 = Task {
			id: 2,
			description: "Task 2".to_string(),
			deadline: Default::default(),
			details: Default::default(),
			completed: false,
		};

		let event1 = TaskEvent {
			id: 2,
			data: TaskEventData::CreateTask(task1),
		};
		let event2 = TaskEvent {
			id: 1,
			data: TaskEventData::CreateTask(task2),
		};

		hashmap.insert(event2.id, event2.clone());
		hashmap.insert(event1.id, event1.clone());

		let sorted_events = hashmap_to_sorted_vec(&hashmap);

		assert_eq!(sorted_events.len(), 2);
		assert_eq!(sorted_events[0].id, 1);
		assert_eq!(sorted_events[1].id, 2);
	}
}
