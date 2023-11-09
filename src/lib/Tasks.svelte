<!-- src/lib/Tasks.svelte -->
<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	import NewTaskForm from "./NewTaskForm.svelte";
	import PasswordForm from "./PasswordForm.svelte";
	import TaskList from "./TaskList.svelte";

	interface Task {
		id: number;
		description: string;
		deadline: number;
		completed: boolean;
	}

	let tasks: Task[] = [];
	let showCompleted = false;
	let isUnlocked = false;

	const unlock = async (password: string) => {
		await invoke("unlock", { password });
		try {
			tasks = await invoke("load_tasks");
			isUnlocked = true;
			console.log("succeeded")
		} catch (error) {
			console.log(`error: ${error}`)
		}
	};

	const addTask = async (description: string, deadline: string) => {
		const trimmedDescription = description.trim();
		const newDeadlineNumber = new Date(deadline).getTime();

		if (trimmedDescription.length === 0 || isNaN(newDeadlineNumber)) {
			return;
		}
		const task: Task = {
			id: Date.now(),
			description: trimmedDescription,
			deadline: newDeadlineNumber,
			completed: false,
		};
		tasks = [...tasks, task];
		tasks.sort((a, b) => a.deadline - b.deadline);
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
	<NewTaskForm {addTask} />
	<TaskList {tasks} {showCompleted} {toggleComplete} {deleteTask} />
{:else}
	<PasswordForm {unlock} />
{/if}
