<script lang="ts">
	import type { Task } from "$lib/model";
	import AddTaskButton from "$lib/component/AddTaskButton.svelte";
	import TaskList from "$lib/component/TaskList.svelte";

	export let addTask: (task: Task) => Promise<void>;
	export let editTask: (task: Task) => Promise<void>;
	export let completeTask: (taskId: number) => Promise<void>;
	export let uncompleteTask: (taskId: number) => Promise<void>;
	export let deleteTask: (taskId: number) => Promise<void>;
	export let tasks: Task[];
</script>

<main class="container">
	<section class="center">
		<AddTaskButton {addTask} />
	</section>
	<section>
		<TaskList {tasks} {completeTask} {uncompleteTask} {editTask} {deleteTask} />
	</section>
</main>

<style>
	.center {
		display: flex;
		justify-content: center;
	}
</style>
