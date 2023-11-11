<script lang="ts">
	import type { Task } from "./types";
	import NewTaskForm from "./NewTaskForm.svelte";
	import TaskList from "./TaskList.svelte";

	export let addTask: (description: string, deadline: string) => Promise<void>;
	export let editTask: (
		taskId: number,
		description: string,
		deadline: string
	) => Promise<void>;
	export let toggleComplete: (taskId: number) => Promise<void>;
	export let deleteTask: (taskId: number) => Promise<void>;
	export let tasks: Task[];
</script>

<NewTaskForm {addTask} />
<br />
<br />
<TaskList {tasks} {toggleComplete} {editTask} {deleteTask} />
