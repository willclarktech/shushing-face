<script lang="ts">
	import type { Task } from "$lib/model";
	import TaskForm from "./TaskForm.svelte";

	export let addTask: (task: Task) => void | Promise<void>;

	let isOpen = false;

	const open = () => {
		isOpen = true;
	};

	const onDone = () => {
		isOpen = false;
	};
</script>

<div>
	<button type="button" on:click={open}>Add a task</button>
	<TaskForm {isOpen} saveTask={addTask} {onDone} />
</div>
