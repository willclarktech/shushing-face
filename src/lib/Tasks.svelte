<script lang="ts">
	import { onMount } from "svelte";

	interface Task {
		id: number;
		description: string;
		deadline: Date;
		completed: boolean;
	}

	let tasks: Task[] = [];
	let newDescription = "";
	let newDeadline = "";
	let showCompleted = false;

	// This function adds a new to-do item
	function addTask() {
		console.log("newDescription", newDescription, typeof newDescription);
		console.log("newDeadline", newDeadline, typeof newDeadline);
		if (newDescription.trim() && newDeadline) {
			const task: Task = {
				id: Date.now(),
				description: newDescription,
				deadline: new Date(newDeadline),
				completed: false,
			};
			tasks = [...tasks, task];
			tasks.sort((a, b) => a.deadline.getTime() - b.deadline.getTime());
			newDescription = "";
			newDeadline = "null";
		}
	}

	function toggleComplete(taskId: number) {
		tasks = tasks.map((task) => {
			if (task.id === taskId) {
				return { ...task, completed: !task.completed };
			}
			return task;
		});
	}

	onMount(() => {
		// Optionally initialize with some data or from localStorage
	});
</script>

<label>
	Description:
	<input type="text" bind:value={newDescription} />
</label>
<label>
	Deadline:
	<input type="date" bind:value={newDeadline} />
</label>
<button on:click={addTask}>Add Task</button>

{#if tasks.length}
	<ul>
		{#each tasks as task (task.id)}
			{#if !task.completed || showCompleted}
				<li class:completed={task.completed}>
					{task.description} - due by {task.deadline.toLocaleDateString()}
					<button on:click={() => toggleComplete(task.id)}>
						{task.completed ? "Undo" : "Complete"}
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
