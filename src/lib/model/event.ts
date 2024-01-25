import type { Task } from "./task";

export enum TaskEventType {
	CreateTask,
	UpdateTask,
	DeleteTask,
}

export type TaskEvent =
	| {
			readonly type: TaskEventType.CreateTask;
			readonly id: number;
			readonly task: Task;
	  }
	| {
			readonly type: TaskEventType.UpdateTask;
			readonly id: number;
			readonly task: Task;
	  }
	| {
			readonly type: TaskEventType.DeleteTask;
			readonly id: number;
			readonly taskId: number;
	  };

/** Rust-friendly format */
export type FormattedTaskEventData =
	| {
			readonly CreateTask: Task;
	  }
	| {
			readonly UpdateTask: Task;
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
		case TaskEventType.UpdateTask: {
			const taskToEdit = tasks.find((t) => t.id === event.task.id);
			if (taskToEdit) {
				Object.assign(taskToEdit, event.task);
			}
			break;
		}
		case TaskEventType.DeleteTask: {
			const indexToDelete = tasks.findIndex((t) => t.id === event.taskId);
			if (indexToDelete !== -1) tasks.splice(indexToDelete, 1);
			break;
		}
	}
	return tasks.sort((a, b) => Date.parse(a.deadline) - Date.parse(b.deadline));
};

export const applyEvents = (
	tasks: Task[],
	events: readonly TaskEvent[]
): Task[] => events.reduce(applyEvent, tasks);

/** Converts to Rust-friendly format */
export const formatEvent = (event: TaskEvent): FormattedTaskEvent => {
	switch (event.type) {
		case TaskEventType.CreateTask:
			return {
				id: event.id,
				data: {
					CreateTask: event.task,
				},
			};
		case TaskEventType.UpdateTask:
			return {
				id: event.id,
				data: {
					UpdateTask: event.task,
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

export const isCreateTaskEvent = (
	event: FormattedTaskEvent
): event is FormattedTaskEvent & {
	readonly data: {
		readonly CreateTask: Task;
	};
} => {
	return Object.keys(event.data).includes("CreateTask");
};

export const isUpdateTaskEvent = (
	event: FormattedTaskEvent
): event is FormattedTaskEvent & {
	readonly data: {
		readonly UpdateTask: Task;
	};
} => {
	return Object.keys(event.data).includes("UpdateTask");
};

export const isDeleteTaskEvent = (
	event: FormattedTaskEvent
): event is FormattedTaskEvent & {
	readonly data: {
		readonly DeleteTask: number;
	};
} => {
	return Object.keys(event.data).includes("DeleteTask");
};

/** Converts from Rust-friendly format */
export const unformatEvent = (event: FormattedTaskEvent): TaskEvent => {
	if (isCreateTaskEvent(event)) {
		return {
			type: TaskEventType.CreateTask,
			id: event.id,
			task: event.data.CreateTask,
		};
	}
	if (isUpdateTaskEvent(event)) {
		return {
			type: TaskEventType.UpdateTask,
			id: event.id,
			task: event.data.UpdateTask,
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
