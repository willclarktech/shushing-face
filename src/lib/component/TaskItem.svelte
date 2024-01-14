<script lang="ts">
	import type { Task } from "$lib/model";
	import Icon from "./Icon.svelte";

	export let task: Task;
	export let completeTask: (taskId: number) => void | Promise<void>;
	export let uncompleteTask: (taskId: number) => void | Promise<void>;
	export let deleteTask: (taskId: number) => void | Promise<void>;
	export let startEditing: (taskId: number) => void | Promise<void>;
</script>

<li class:completed={task.completed} class="task">
	<span class="task-description">
		<label>
			<input
				type="checkbox"
				checked={task.completed}
				on:change={() =>
					task.completed ? uncompleteTask(task.id) : completeTask(task.id)}
			/>
			{task.description}
		</label>
	</span>
	<span class="task-actions">
		<button on:click={() => startEditing(task.id)}>
			<Icon variant="edit" />
		</button>
		<button on:click={() => deleteTask(task.id)}>
			<Icon variant="trash" />
		</button>
	</span>
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
		justify-content: space-between;
		align-items: center;
	}

	.task-actions {
		display: flex;
		gap: 8px;
	}

	.completed {
		text-decoration: line-through;
	}
</style>
