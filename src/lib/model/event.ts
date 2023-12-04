import type { Task } from "$lib/model/task";

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
			readonly id: number;
			readonly task: Task;
	  }
	| {
			readonly type: TaskEventType.CompleteTask;
			readonly id: number;
			readonly taskId: number;
	  }
	| {
			readonly type: TaskEventType.UncompleteTask;
			readonly id: number;
			readonly taskId: number;
	  }
	| {
			readonly type: TaskEventType.EditTask;
			readonly id: number;
			readonly taskId: number;
			readonly edit: Pick<Task, "description" | "deadline" | "details">;
	  }
	| {
			readonly type: TaskEventType.DeleteTask;
			readonly id: number;
			readonly taskId: number;
	  };

export type FormattedTaskEventData =
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

export type FormattedTaskEvent = {
	readonly id: number;
	readonly data: FormattedTaskEventData;
};

export const applyEvent = (tasks: Task[], event: TaskEvent): Task[] => {
	switch (event.type) {
		case TaskEventType.CreateTask:
			tasks.push(event.task);
			break;
		case TaskEventType.CompleteTask: {
			const completeTask = tasks.find((t) => t.id === event.taskId);
			if (completeTask) completeTask.completed = true;
			break;
		}
		case TaskEventType.UncompleteTask: {
			const uncompleteTask = tasks.find((t) => t.id === event.taskId);
			if (uncompleteTask) uncompleteTask.completed = false;
			break;
		}
		case TaskEventType.EditTask: {
			const taskToEdit = tasks.find((t) => t.id === event.taskId);
			if (taskToEdit) {
				Object.assign(taskToEdit, event.edit);
			}
			break;
		}
		case TaskEventType.DeleteTask: {
			const indexToDelete = tasks.findIndex((t) => t.id === event.taskId);
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
				id: event.id,
				data: {
					CreateTask: event.task,
				},
			};
		case TaskEventType.CompleteTask:
			return {
				id: event.id,
				data: {
					CompleteTask: event.taskId,
				},
			};
		case TaskEventType.UncompleteTask:
			return {
				id: event.id,
				data: {
					UncompleteTask: event.taskId,
				},
			};
		case TaskEventType.EditTask:
			return {
				id: event.id,
				data: {
					EditTask: [event.taskId, event.edit],
				},
			};
		case TaskEventType.DeleteTask:
			return {
				id: event.id,
				data: {
					DeleteTask: event.taskId,
				},
			};
		default:
			throw new Error(`Unrecognized task event: ${event}`);
	}
};

const isCreateTaskEvent = (
	event: FormattedTaskEvent
): event is FormattedTaskEvent & {
	readonly data: {
		readonly CreateTask: Task;
	};
} => {
	return Object.keys(event.data).includes("CreateTask");
};
const isCompleteTaskEvent = (
	event: FormattedTaskEvent
): event is FormattedTaskEvent & {
	readonly data: {
		readonly CompleteTask: number;
	};
} => {
	return Object.keys(event.data).includes("CompleteTask");
};
const isUncompleteTaskEvent = (
	event: FormattedTaskEvent
): event is FormattedTaskEvent & {
	readonly data: {
		readonly UncompleteTask: number;
	};
} => {
	return Object.keys(event.data).includes("UncompleteTask");
};
const isEditTaskEvent = (
	event: FormattedTaskEvent
): event is FormattedTaskEvent & {
	readonly data: {
		readonly EditTask: readonly [
			number,
			Pick<Task, "description" | "deadline" | "details">
		];
	};
} => {
	return Object.keys(event.data).includes("EditTask");
};
const isDeleteTaskEvent = (
	event: FormattedTaskEvent
): event is FormattedTaskEvent & {
	readonly data: {
		readonly DeleteTask: number;
	};
} => {
	return Object.keys(event.data).includes("DeleteTask");
};

export const unformatEvent = (event: FormattedTaskEvent): TaskEvent => {
	if (isCreateTaskEvent(event)) {
		return {
			type: TaskEventType.CreateTask,
			id: event.id,
			task: event.data.CreateTask,
		};
	}
	if (isCompleteTaskEvent(event)) {
		return {
			type: TaskEventType.CompleteTask,
			id: event.id,
			taskId: event.data.CompleteTask,
		};
	}
	if (isUncompleteTaskEvent(event)) {
		return {
			type: TaskEventType.UncompleteTask,
			id: event.id,
			taskId: event.data.UncompleteTask,
		};
	}
	if (isEditTaskEvent(event)) {
		return {
			type: TaskEventType.EditTask,
			id: event.id,
			taskId: event.data.EditTask[0],
			edit: event.data.EditTask[1],
		};
	}
	if (isDeleteTaskEvent(event)) {
		return {
			type: TaskEventType.DeleteTask,
			id: event.id,
			taskId: event.data.DeleteTask,
		};
	}
	throw new Error(`Unrecognized task event: ${event}`);
};
