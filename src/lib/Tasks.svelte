<script lang="ts">
	import type { Task } from "./types";
	import AddTaskForm from "./AddTaskForm.svelte";
	import TaskList from "./TaskList.svelte";

	export let addTask: (
		description: string,
		deadline: string,
		details: string
	) => Promise<void>;
	export let editTask: (
		taskId: number,
		description: string,
		deadline: string,
		details: string
	) => Promise<void>;
	export let toggleComplete: (taskId: number) => Promise<void>;
	export let deleteTask: (taskId: number) => Promise<void>;
	export let tasks: Task[];
</script>

<div class="container">
	<section>
		<AddTaskForm {addTask} />
	</section>
	<section>
		<TaskList {tasks} {toggleComplete} {editTask} {deleteTask} />
	</section>
</div>

<style>
	.container {
		max-width: 80vw;
	}
</style>
