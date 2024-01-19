<script lang="ts">
	import type { Task } from "$lib/model";
	import TaskList from "$lib/component/TaskList.svelte";

	export let editTask: (task: Task) => Promise<void>;
	export let completeTask: (taskId: number) => Promise<void>;
	export let uncompleteTask: (taskId: number) => Promise<void>;
	export let deleteTask: (taskId: number) => Promise<void>;
	export let tasks: Task[];
</script>

<section>
	<TaskList {tasks} {completeTask} {uncompleteTask} {editTask} {deleteTask} />
</section>
