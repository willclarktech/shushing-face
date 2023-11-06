<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri'
	import { onMount } from "svelte";

	interface Task {
		id: number;
		description: string;
		deadline: number;
		completed: boolean;
	}

	let tasks: Task[] = [];
	let newDescription = "";
	let newDeadline = "";
	let showCompleted = false;

	const addTask = () => {
		const trimmedDescription = newDescription.trim();
		const newDeadlineNumber = new Date(newDeadline).getTime();

		if (trimmedDescription.length === 0 || isNaN(newDeadlineNumber)) {
			return;
		}
		const task: Task = {
			id: Date.now(),
			description: newDescription,
			deadline: newDeadlineNumber,
			completed: false,
		};
		tasks = [...tasks, task];
		tasks.sort((a, b) => a.deadline - b.deadline);
		newDescription = "";
		newDeadline = "";
	};

	const toggleComplete = (taskId: number) => {
		tasks = tasks.map((task) => {
			if (task.id !== taskId) {
				return task;
			}
			return { ...task, completed: !task.completed };
		});
	};

	onMount(async () => {
		tasks = await invoke('load_tasks');
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
					{task.description} - due by {new Date(task.deadline).toLocaleDateString()}
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
