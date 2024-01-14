<script lang="ts">
	import type { Task } from "$lib/model";
	import Icon from "$lib/component/Icon.svelte";
	import TaskForm from "$lib/component/TaskForm.svelte";
	import TaskItem from "$lib/component/TaskItem.svelte";

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
		}, new Map<number | DateGroup, readonly Task[]>());

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
								<TaskForm
									{task}
									isOpen={true}
									saveTask={editTask}
									cancel={stopEditing}
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
		</section>
	{:else}
		<section class="all-done">
			<div><Icon variant="sun" /></div>
		</section>
	{/if}
</div>

<style>
	.all-done {
		display: flex;
		justify-content: center;
		font-size: 10rem;
	}
</style>
