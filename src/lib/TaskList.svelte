<script lang="ts">
	import type { Task } from "./types";

	export let tasks: Task[];
	export let toggleComplete: (taskId: number) => void | Promise<void>;
	export let editTask: (
		taskId: number,
		description: string,
		deadline: string
	) => void | Promise<void>;
	export let deleteTask: (taskId: number) => void | Promise<void>;

	let showCompleted = false;
	let taskUnderEdit: number | null = null;
	let editedDescription: string = "";
	let editedDeadline: string = "";

	const startEditing = (task: Task) => {
		editedDescription = task.description;
		editedDeadline = new Date(task.deadline).toDateString();
		taskUnderEdit = task.id;
	};

	const submitEdit = () => {
		if (taskUnderEdit !== null) {
			editTask(taskUnderEdit, editedDescription, editedDeadline);
		}
		stopEditing();
	};

	const stopEditing = () => {
		taskUnderEdit = null;
	};
</script>

{#if tasks.length}
	<ul>
		{#each tasks as task (task.id)}
			{#if !task.completed || showCompleted}
				{#if task.id === taskUnderEdit}
					<li>
						<form on:submit|preventDefault={submitEdit}>
							<input type="text" bind:value={editedDescription} />
							<input type="date" bind:value={editedDeadline} />
							<button type="submit">💾</button>
							<button type="button" on:click={stopEditing}>❌</button>
						</form>
					</li>
				{:else}
					<li class:completed={task.completed}>
						{task.description} - due by {new Date(
							task.deadline
						).toLocaleDateString()}
						<button on:click={() => toggleComplete(task.id)}>
							{task.completed ? "↩️" : "✔️"}
						</button>
						<button on:click={() => startEditing(task)}> ✏️ </button>
						<button on:click={() => deleteTask(task.id)}> 🗑️ </button>
					</li>
				{/if}
			{/if}
		{/each}
	</ul>
{:else}
	<p>🤷</p>
{/if}

<label>
	<input type="checkbox" bind:checked={showCompleted} />
	Show completed tasks
</label>

<style>
	.completed {
		text-decoration: line-through;
	}
</style>
