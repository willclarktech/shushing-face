<script lang="ts">
	export let addTask: (
		description: string,
		deadline: string,
		details: string
	) => void | Promise<void>;
	let newDescription = "";
	let newDetails = "";
	let newDeadline = new Date().toDateString();
	let isOpen = false;

	const open = () => {
		isOpen = true;
	};
	const close = () => {
		isOpen = false;
	};

	const submit = () => {
		void addTask(newDescription, newDeadline, newDetails);
		newDescription = "";
		newDetails = "";
		newDeadline = new Date().toDateString();
	};
</script>

<div class="container-narrow">
	<button on:click={open}>Add a task</button>
	<dialog open={isOpen}>
		<div class="container-narrow">
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
						<button type="submit">Save</button>
						<button type="button" class="secondary" on:click={close}>
							Cancel
						</button>
					</div>
				</fieldset>
			</form>
		</div>
	</dialog>
</div>
