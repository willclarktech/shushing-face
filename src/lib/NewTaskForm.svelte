<script lang="ts">
	export let addTask: (
		description: string,
		deadline: string
	) => void | Promise<void>;
	let newDescription = "";
	let newDeadline = new Date().toDateString();

	const submit = () => {
		void addTask(newDescription, newDeadline);
		newDescription = "";
		newDeadline = new Date().toDateString();
	};
</script>

<label>
	Description:
	<input type="text" bind:value={newDescription} />
</label>
<label>
	Deadline:
	<input type="date" bind:value={newDeadline} />
</label>
<button on:click={submit}>Add Task</button>
