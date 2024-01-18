<script lang="ts">
	import type { Task } from "$lib/model";
	import Icon from "./Icon.svelte";

	export let task: Task;
	export let completeTask: (taskId: number) => void | Promise<void>;
	export let uncompleteTask: (taskId: number) => void | Promise<void>;
	export let deleteTask: (taskId: number) => void | Promise<void>;
	export let startEditing: (taskId: number) => void | Promise<void>;

	$: taskInputId = `task-${task.id}`;
</script>

<li class:completed={task.completed} class="task">
	<fieldset>
		<label for={taskInputId}>
			<input
				name={taskInputId}
				type="checkbox"
				checked={task.completed}
				on:change={() =>
					task.completed ? uncompleteTask(task.id) : completeTask(task.id)}
			/>
			{task.description}
		</label>
	</fieldset>
	<fieldset role="group">
		<button type="button" on:click={() => startEditing(task.id)}>
			<Icon variant="edit" />
		</button>
		<button type="button" on:click={() => deleteTask(task.id)}>
			<Icon variant="trash" />
		</button>
	</fieldset>
</li>
{#if task.details.length > 0}
	<details>
		<summary>Details</summary>
		{#each task.details.split("\n") as line}
			<span>&nbsp;{line}</span>
			<br />
		{/each}
	</details>
{/if}

<style>
	.task {
		display: flex;
		align-items: center;
	}

	.completed {
		text-decoration: line-through !important;
	}
</style>
