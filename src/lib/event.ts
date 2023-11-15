import type { Task } from "$lib/task";

export enum TaskEventType {
	CreateTask,
	CompleteTask,
	UncompleteTask,
	EditTask,
	DeleteTask,
}

export type TaskEvent =
	| {
			readonly type: TaskEventType.CreateTask;
			readonly task: Task;
	  }
	| {
			readonly type: TaskEventType.CompleteTask;
			readonly id: number;
	  }
	| {
			readonly type: TaskEventType.UncompleteTask;
			readonly id: number;
	  }
	| {
			readonly type: TaskEventType.EditTask;
			readonly id: number;
			readonly edit: Pick<Task, "description" | "deadline" | "details">;
	  }
	| {
			readonly type: TaskEventType.DeleteTask;
			readonly id: number;
	  };

export type FormattedTaskEvent =
	| {
			readonly CreateTask: Task;
	  }
	| {
			readonly CompleteTask: number;
	  }
	| {
			readonly UncompleteTask: number;
	  }
	| {
			readonly EditTask: readonly [
				number,
				Pick<Task, "description" | "deadline" | "details">
			];
	  }
	| {
			readonly DeleteTask: number;
	  };

export const applyEvent = (tasks: Task[], event: TaskEvent): Task[] => {
	switch (event.type) {
		case TaskEventType.CreateTask:
			tasks.push(event.task);
			break;
		case TaskEventType.CompleteTask: {
			const completeTask = tasks.find((t) => t.id === event.id);
			if (completeTask) completeTask.completed = true;
			break;
		}
		case TaskEventType.UncompleteTask: {
			const uncompleteTask = tasks.find((t) => t.id === event.id);
			if (uncompleteTask) uncompleteTask.completed = false;
			break;
		}
		case TaskEventType.EditTask: {
			const taskToEdit = tasks.find((t) => t.id === event.id);
			if (taskToEdit) {
				Object.assign(taskToEdit, event.edit);
			}
			break;
		}
		case TaskEventType.DeleteTask: {
			const indexToDelete = tasks.findIndex((t) => t.id === event.id);
			if (indexToDelete !== -1) tasks.splice(indexToDelete, 1);
			break;
		}
	}
	return tasks.sort((a, b) => a.deadline - b.deadline);
};

export const applyEvents = (
	tasks: Task[],
	events: readonly TaskEvent[]
): Task[] => events.reduce(applyEvent, tasks);

// Converts to Rust-friendly format
export const formatEvent = (event: TaskEvent): FormattedTaskEvent => {
	switch (event.type) {
		case TaskEventType.CreateTask:
			return {
				CreateTask: event.task,
			};
		case TaskEventType.CompleteTask:
			return {
				CompleteTask: event.id,
			};
		case TaskEventType.UncompleteTask:
			return {
				UncompleteTask: event.id,
			};
		case TaskEventType.EditTask:
			return {
				EditTask: [event.id, event.edit],
			};
		case TaskEventType.DeleteTask:
			return {
				DeleteTask: event.id,
			};
		default:
			throw new Error(`Unrecognized task event: ${event}`);
	}
};

const isCreateTaskEvent = (
	event: FormattedTaskEvent
): event is {
	readonly CreateTask: Task;
} => {
	return Object.keys(event).includes("CreateTask");
};
const isCompleteTaskEvent = (
	event: FormattedTaskEvent
): event is {
	readonly CompleteTask: number;
} => {
	return Object.keys(event).includes("CompleteTask");
};
const isUncompleteTaskEvent = (
	event: FormattedTaskEvent
): event is {
	readonly UncompleteTask: number;
} => {
	return Object.keys(event).includes("UncompleteTask");
};
const isEditTaskEvent = (
	event: FormattedTaskEvent
): event is {
	readonly EditTask: readonly [
		number,
		Pick<Task, "description" | "deadline" | "details">
	];
} => {
	return Object.keys(event).includes("EditTask");
};
const isDeleteTaskEvent = (
	event: FormattedTaskEvent
): event is {
	readonly DeleteTask: number;
} => {
	return Object.keys(event).includes("DeleteTask");
};

export const unformatEvent = (event: FormattedTaskEvent): TaskEvent => {
	if (isCreateTaskEvent(event)) {
		return {
			type: TaskEventType.CreateTask,
			task: event.CreateTask,
		};
	}
	if (isCompleteTaskEvent(event)) {
		return {
			type: TaskEventType.CompleteTask,
			id: event.CompleteTask,
		};
	}
	if (isUncompleteTaskEvent(event)) {
		return {
			type: TaskEventType.UncompleteTask,
			id: event.UncompleteTask,
		};
	}
	if (isEditTaskEvent(event)) {
		return {
			type: TaskEventType.EditTask,
			id: event.EditTask[0],
			edit: event.EditTask[1],
		};
	}
	if (isDeleteTaskEvent(event)) {
		return {
			type: TaskEventType.DeleteTask,
			id: event.DeleteTask,
		};
	}
	throw new Error(`Unrecognized task event: ${event}`);
};
