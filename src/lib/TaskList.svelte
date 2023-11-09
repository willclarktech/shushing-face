<script lang="ts">
	export let tasks: Task[];
	export let showCompleted: boolean;
	export let toggleComplete: (taskId: number) => void | Promise<void>;
	export let deleteTask: (taskId: number) => void | Promise<void>;

	interface Task {
		id: number;
		description: string;
		deadline: number;
		completed: boolean;
	}
</script>

{#if tasks.length}
	<ul>
		{#each tasks as task (task.id)}
			{#if !task.completed || showCompleted}
				<li class:completed={task.completed}>
					{task.description} - due by {new Date(
						task.deadline
					).toLocaleDateString()}
					<button on:click={() => toggleComplete(task.id)}>
						{task.completed ? "↩️" : "✔️"}
					</button>
					<button on:click={() => deleteTask(task.id)}>
						{"🗑️"}
					</button>
				</li>
			{/if}
		{/each}
	</ul>
{:else}
	<p>No to-dos yet!</p>
{/if}

<label>
	<input type="checkbox" bind:checked={showCompleted} />
	Show completed to-dos
</label>

<style>
	.completed {
		text-decoration: line-through;
	}
</style>
