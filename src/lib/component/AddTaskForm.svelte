<script lang="ts">
	import Icon from "$lib/component/Icon.svelte";

	export let addTask: (
		description: string,
		deadline: string,
		details: string
	) => void | Promise<void>;
	let newDescription = "";
	let newDetails = "";
	let newDeadline = new Date().toDateString();
	let showForm = false;

	const submit = () => {
		void addTask(newDescription, newDeadline, newDetails);
		newDescription = "";
		newDetails = "";
		newDeadline = new Date().toDateString();
	};
</script>

<div class="container-narrow">
	{#if showForm}
		<form on:submit|preventDefault={submit}>
			<fieldset>
				<input
					id="description"
					name="description"
					type="text"
					placeholder="What needs to be done?"
					bind:value={newDescription}
				/>
				<input
					id="deadline"
					name="deadline"
					type="date"
					bind:value={newDeadline}
				/>
				<details>
					<summary>Details</summary>
					<textarea
						id="details"
						name="details"
						rows="5"
						placeholder="Details"
						bind:value={newDetails}
					/>
				</details>
				<div class="grid">
					<button type="submit">
						<Icon variant="plus" />
						Add task
					</button>
					<button
						class="secondary"
						on:click={() => {
							showForm = false;
						}}
					>
						<Icon variant="times" />
						Cancel
					</button>
				</div>
			</fieldset>
		</form>
	{:else}
		<button
			on:click={() => {
				showForm = true;
			}}
		>
			<Icon variant="plus" />
			Add a task
		</button>
	{/if}
</div>
