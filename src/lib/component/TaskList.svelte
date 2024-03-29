<script lang="ts">
	import type { Task } from "$lib/model";
	import {
		DateGroup,
		getDayAfterTomorrow,
		getToday,
		getTomorrow,
	} from "$lib/model";
	import Icon from "./Icon.svelte";
	import TaskForm from "./TaskForm.svelte";
	import TaskItem from "./TaskItem.svelte";

	export let tasks: Task[];
	export let completeTask: (taskId: number) => void | Promise<void>;
	export let uncompleteTask: (taskId: number) => void | Promise<void>;
	export let editTask: (task: Task) => void | Promise<void>;
	export let deleteTask: (taskId: number) => void | Promise<void>;

	let showCompleted = false;
	let taskUnderEdit: number | null = null;

	const startEditing = (taskId: number) => {
		taskUnderEdit = taskId;
	};

	const stopEditing = () => {
		taskUnderEdit = null;
	};

	const groupTasksByDate = (tasks: readonly Task[]) =>
		tasks.reduce((accumulator, task) => {
			const key =
				task.deadline < getToday()
					? DateGroup.Past
					: task.deadline < getTomorrow()
					? DateGroup.Today
					: task.deadline < getDayAfterTomorrow()
					? DateGroup.Tomorrow
					: task.deadline;

			const previous = accumulator.get(key) ?? [];
			accumulator.set(key, [...previous, task]);
			return accumulator;
		}, new Map<string | DateGroup, readonly Task[]>());

	$: filteredTasks = showCompleted
		? tasks
		: tasks.filter((task) => !task.completed);
	$: groupedTasks = groupTasksByDate(filteredTasks);
	$: dateGroups = [...groupedTasks.keys()];
</script>

<section>
	<label>
		<input type="checkbox" role="switch" bind:checked={showCompleted} />
		Show completed tasks
	</label>
</section>
<section>
	{#if filteredTasks.length}
		<ul>
			{#each dateGroups as date}
				<h3>
					{date in DateGroup ? date : new Date(date).toLocaleDateString()}
				</h3>
				{#each groupedTasks.get(date) ?? [] as task (task.id)}
					{#if !task.completed || showCompleted}
						{#if task.id === taskUnderEdit}
							<TaskForm
								{task}
								isOpen={true}
								saveTask={editTask}
								onDone={stopEditing}
							/>
						{:else}
							<TaskItem
								{task}
								{completeTask}
								{uncompleteTask}
								{deleteTask}
								{startEditing}
							/>
						{/if}
					{/if}
				{/each}
			{/each}
		</ul>
	{:else}
		<div class="all-done"><Icon variant="sun" /></div>
	{/if}
</section>

<style>
	.all-done {
		display: flex;
		justify-content: center;
		font-size: 10rem;
	}
</style>
