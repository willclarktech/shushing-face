<script lang="ts">
	import type { Task } from "$lib/model";
	import AddTaskForm from "$lib/component/AddTaskForm.svelte";
	import TaskList from "$lib/component/TaskList.svelte";

	export let addTask: (task: Task) => Promise<void>;
	export let editTask: (task: Task) => Promise<void>;
	export let completeTask: (taskId: number) => Promise<void>;
	export let uncompleteTask: (taskId: number) => Promise<void>;
	export let deleteTask: (taskId: number) => Promise<void>;
	export let tasks: Task[];
</script>

<div class="container">
	<section>
		<AddTaskForm {addTask} />
	</section>
	<section>
		<TaskList {tasks} {completeTask} {uncompleteTask} {editTask} {deleteTask} />
	</section>
</div>

<style>
	.container {
		max-width: 80vw;
	}
</style>
