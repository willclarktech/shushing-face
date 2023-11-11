<script lang="ts">
	export let addTask: (
		description: string,
		deadline: string,
		details: string
	) => void | Promise<void>;
	let newDescription = "";
	let newDetails = "";
	let newDeadline = new Date().toDateString();

	const submit = () => {
		void addTask(newDescription, newDeadline, newDetails);
		newDescription = "";
		newDetails = "";
		newDeadline = new Date().toDateString();
	};
</script>

<label>
	🏷️:
	<input type="text" bind:value={newDescription} />
</label>
<label>
	📆:
	<input type="date" bind:value={newDeadline} />
</label>
<label>
	📝:
	<textarea rows="5" bind:value={newDetails} />
</label>
<button on:click={submit}>🆕</button>
