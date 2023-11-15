<script lang="ts">
	import Icon from "$lib/component/Icon.svelte";
	import type { Task } from "$lib/task";

	export let task: Task;
	export let submitEdit: (
		description: string,
		deadline: string,
		details: string
	) => void | Promise<void>;
	export let stopEditing: () => void | Promise<void>;

	let editedDescription: string = task.description;
	let editedDeadline: string = new Date(task.deadline).toDateString();
	let editedDetails: string = task.details;

	const submit = () => {
		submitEdit(editedDescription, editedDeadline, editedDetails);
	};
</script>

<form on:submit|preventDefault={submit}>
	<input type="text" bind:value={editedDescription} />
	<input type="date" bind:value={editedDeadline} />
	<details>
		<summary>Details</summary>
		<textarea rows="5" bind:value={editedDetails} />
	</details>
	<div class="grid">
		<button type="submit">
			<Icon variant="floppy" />
			Save changes
		</button>
		<button type="button" on:click={stopEditing} class="secondary">
			<Icon variant="times" />
			Cancel
		</button>
	</div>
</form>
