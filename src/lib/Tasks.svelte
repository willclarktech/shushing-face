<!-- src/lib/Tasks.svelte -->
<script lang="ts">
	import NewTaskForm from "./NewTaskForm.svelte";
	import TaskList from "./TaskList.svelte";
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";

	interface Task {
		id: number;
		description: string;
		deadline: number;
		completed: boolean;
	}

	let tasks: Task[] = [];
	let showCompleted = false;

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
		tasks.push(task);
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

	onMount(async () => {
		tasks = await invoke("load_tasks");
	});
</script>

<NewTaskForm {addTask} />
<TaskList {tasks} {showCompleted} {toggleComplete} {deleteTask} />
