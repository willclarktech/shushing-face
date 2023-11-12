<script lang="ts">
	import Icon from "./Icon.svelte";
	import EditTaskForm from "./EditTaskForm.svelte";
	import type { Task } from "./types";

	export let tasks: Task[];
	export let toggleComplete: (taskId: number) => void | Promise<void>;
	export let editTask: (
		taskId: number,
		description: string,
		deadline: string,
		details: string
	) => void | Promise<void>;
	export let deleteTask: (taskId: number) => void | Promise<void>;

	let showCompleted = false;
	let taskUnderEdit: number | null = null;

	const startEditing = (task: Task) => {
		taskUnderEdit = task.id;
	};

	const submitEdit = (
		editedDescription: string,
		editedDeadline: string,
		editedDetails: string
	) => {
		if (taskUnderEdit !== null) {
			editTask(taskUnderEdit, editedDescription, editedDeadline, editedDetails);
		}
		stopEditing();
	};

	const stopEditing = () => {
		taskUnderEdit = null;
	};

	enum DateGroup {
		Past = "Past",
		Today = "Today",
		Tomorrow = "Tomorrow",
	}
	const today = new Date();
	today.setHours(0, 0, 0, 0);
	const tomorrow = new Date(today);
	tomorrow.setDate(today.getDate() + 1);
	const dayAfterTomorrow = new Date(tomorrow);
	dayAfterTomorrow.setDate(tomorrow.getDate() + 1);

	const groupTasksByDate = (tasks: readonly Task[]) =>
		tasks.reduce((accumulator, task) => {
			const key =
				task.deadline < today.getTime()
					? DateGroup.Past
					: task.deadline < tomorrow.getTime()
					? DateGroup.Today
					: task.deadline < dayAfterTomorrow.getTime()
					? DateGroup.Tomorrow
					: task.deadline;

			const previous = accumulator.get(key) ?? [];
			accumulator.set(key, [...previous, task]);
			return accumulator;
		}, new Map());

	$: filteredTasks = showCompleted
		? tasks
		: tasks.filter((task) => !task.completed);
	$: groupedTasks = groupTasksByDate(filteredTasks);
	$: dateGroups = [...groupedTasks.keys()];
</script>

<div>
	<section>
		<label>
			<input type="checkbox" role="switch" bind:checked={showCompleted} />
			Show completed tasks
		</label>
	</section>
	{#if filteredTasks.length}
		<section>
			<ul>
				{#each dateGroups as date}
					<h3>
						{date in DateGroup ? date : new Date(date).toLocaleDateString()}
					</h3>
					{#each groupedTasks.get(date) ?? [] as task (task.id)}
						{#if !task.completed || showCompleted}
							{#if task.id === taskUnderEdit}
								<EditTaskForm {task} {submitEdit} {stopEditing} />
							{:else}
								<li class:completed={task.completed} class="task">
									<span class="task-description">
										<label>
											<input
												type="checkbox"
												checked={task.completed}
												on:change={() => toggleComplete(task.id)}
											/>
											{task.description}
										</label>
									</span>
									<span class="task-actions">
										<button on:click={() => startEditing(task)}>
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
							{/if}
						{/if}
					{/each}
				{/each}
			</ul>
		</section>
	{:else}
		<section class="all-done">
			<div><Icon variant="sun" /></div>
		</section>
	{/if}
</div>

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

	.all-done {
		display: flex;
		justify-content: center;
		font-size: 10rem;
	}
</style>
