import { describe, expect, it } from "vitest";
import {
	type TaskEvent,
	TaskEventType,
	applyEvent,
	applyEvents,
	formatEvent,
	unformatEvent,
	isCreateTaskEvent,
	isUpdateTaskEvent,
	isDeleteTaskEvent,
} from "./event";
import type { Task } from "./task";

const mockTask: Task = {
	id: 1,
	description: "Test Task",
	deadline: "2023-01-01",
	details: "Test Details",
	completed: false,
};

const mockTasks: Task[] = [mockTask];

describe("applyEvent", () => {
	it("applies CreateTask event correctly", () => {
		const newTask: Task = { ...mockTask, id: 2 };
		const event: TaskEvent = {
			type: TaskEventType.CreateTask,
			id: 2,
			task: newTask,
		};

		const updatedTasks = applyEvent([...mockTasks], event);
		expect(updatedTasks).toContainEqual(newTask);
	});

	it("applies UpdateTask event correctly", () => {
		const updatedTask: Task = { ...mockTask, description: "Updated Test Task" };
		const event: TaskEvent = {
			type: TaskEventType.UpdateTask,
			id: 1,
			task: updatedTask,
		};

		const updatedTasks = applyEvent([...mockTasks], event);
		expect(updatedTasks.find((task) => task.id === mockTask.id)).toEqual(
			updatedTask
		);
	});

	it("applies DeleteTask event correctly", () => {
		const event: TaskEvent = {
			type: TaskEventType.DeleteTask,
			id: 1,
			taskId: mockTask.id,
		};

		const updatedTasks = applyEvent([...mockTasks], event);
		expect(updatedTasks).not.toContainEqual(mockTask);
	});
});

describe("applyEvents", () => {
	it("applies a series of events correctly", () => {
		const newTask: Task = { ...mockTask, id: 2, description: "New Test Task" };
		const events: TaskEvent[] = [
			{ type: TaskEventType.CreateTask, id: 2, task: { ...newTask } },
			{
				type: TaskEventType.UpdateTask,
				id: 2,
				task: { ...newTask, description: "Updated" },
			},
			{ type: TaskEventType.DeleteTask, id: 1, taskId: 1 },
		];

		const updatedTasks = applyEvents([...mockTasks], events);
		expect(updatedTasks).not.toContainEqual(newTask);
		expect(updatedTasks).toContainEqual({ ...newTask, description: "Updated" });
		expect(updatedTasks).not.toContainEqual(mockTask);
	});
});

describe("formatEvent", () => {
	it("formats event correctly", () => {
		const event: TaskEvent = {
			type: TaskEventType.CreateTask,
			id: 1,
			task: mockTask,
		};

		const formattedEvent = formatEvent(event);
		expect(formattedEvent).toEqual({
			id: 1,
			data: { CreateTask: mockTask },
		});
	});
});

describe("unformatEvent", () => {
	it("unformats event correctly", () => {
		const formattedEvent = {
			id: 1,
			data: { CreateTask: mockTask },
		};

		const event = unformatEvent(formattedEvent);
		expect(event).toEqual({
			type: TaskEventType.CreateTask,
			id: 1,
			task: mockTask,
		});
	});
});

describe("isCreateTaskEvent", () => {
	it("detects CreateTask event correctly", () => {
		const formattedEvent = {
			id: 1,
			data: { CreateTask: mockTask },
		};

		expect(isCreateTaskEvent(formattedEvent)).toBe(true);
	});
});

describe("isUpdateTaskEvent", () => {
	it("detects UpdateTask event correctly", () => {
		const formattedEvent = {
			id: 1,
			data: { UpdateTask: mockTask },
		};

		expect(isUpdateTaskEvent(formattedEvent)).toBe(true);
	});
});

describe("isDeleteTaskEvent", () => {
	it("detects DeleteTask event correctly", () => {
		const formattedEvent = {
			id: 1,
			data: { DeleteTask: 1 },
		};

		expect(isDeleteTaskEvent(formattedEvent)).toBe(true);
	});
});
