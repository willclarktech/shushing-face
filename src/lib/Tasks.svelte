<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	import type { Task } from "./types";
	import NewTaskForm from "./NewTaskForm.svelte";
	import UnlockForm from "./UnlockForm.svelte";
	import TaskList from "./TaskList.svelte";

	let tasks: Task[] = [];
	let showCompleted = false;
	let isUnlocked = false;

	const unlock = async (password: string) => {
		await invoke("unlock", { password });
		try {
			tasks = await invoke("load_tasks");
			isUnlocked = true;
		} catch (error) {
			console.log(`error: ${error}`);
		}
	};

	const lock = async () => {
		await invoke("lock");
		tasks = [];
		isUnlocked = false;
	};

	const createTask = (
		id: number,
		description: string,
		deadline: string,
		completed = false
	): Task => {
		const trimmedDescription = description.trim();
		const deadlineNumber = new Date(deadline).getTime();

		if (trimmedDescription.length === 0 || isNaN(deadlineNumber)) {
			throw new Error("Invalid task info");
		}
		return {
			id,
			description: trimmedDescription,
			deadline: deadlineNumber,
			completed,
		};
	};

	const addTask = async (description: string, deadline: string) => {
		const task = createTask(Date.now(), description, deadline);
		tasks = [...tasks, task].sort((a, b) => a.deadline - b.deadline);
		await invoke("save_tasks", { tasks });
	};

	const editTask = async (
		taskId: number,
		description: string,
		deadline: string
	) => {
		tasks = tasks
			.map((task) => {
				if (task.id !== taskId) {
					return task;
				}
				return createTask(taskId, description, deadline, task.completed);
			})
			.sort((a, b) => a.deadline - b.deadline);
		await invoke("save_tasks", { tasks });
	};

	const toggleComplete = async (taskId: number) => {
		tasks = tasks.map((task) => {
			if (task.id !== taskId) {
				return task;
			}
			return { ...task, completed: !task.completed };
		});
		await invoke("save_tasks", { tasks });
	};

	const deleteTask = async (taskId: number) => {
		tasks = tasks.filter((task) => task.id !== taskId);
		await invoke("save_tasks", { tasks });
	};
</script>

{#if isUnlocked}
	<button on:click={lock}>Lock</button>
	<br />
	<br />
	<NewTaskForm {addTask} />
	<br />
	<br />
	<TaskList {tasks} {showCompleted} {toggleComplete} {editTask} {deleteTask} />
{:else}
	<UnlockForm {unlock} />
{/if}
