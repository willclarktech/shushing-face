<script lang="ts">
	import type { Task } from "$lib/model";
	import { getToday } from "$lib/model";

	export let task: Task | null = null;
	export let isOpen: boolean;
	export let saveTask: (task: Task) => void | Promise<void>;
	export let cancel: () => void;

	let description = task?.description ?? "";
	let details = task?.details ?? "";
	let deadline = task?.deadline ?? getToday();

	const submit = () => {
		const trimmedDescription = description.trim();
		if (trimmedDescription.length === 0) {
			throw new Error("Invalid task info");
		}
		const taskToSave = {
			id: task?.id ?? Date.now(),
			description: trimmedDescription,
			details: details.trim(),
			deadline: deadline,
			completed: task?.completed ?? false,
		};
		saveTask(taskToSave);
		description = "";
		details = "";
		deadline = getToday();
		cancel();
	};
</script>

<dialog open={isOpen}>
	<div class="container-narrow">
		<form on:submit|preventDefault={submit}>
			<fieldset>
				<input
					id="description"
					name="description"
					type="text"
					placeholder="What needs to be done?"
					bind:value={description}
				/>
				<input
					id="deadline"
					name="deadline"
					type="date"
					bind:value={deadline}
				/>
				<details>
					<summary>Details</summary>
					<textarea
						id="details"
						name="details"
						rows="5"
						placeholder="Details"
						bind:value={details}
					/>
				</details>
			</fieldset>

			<div class="grid">
				<button type="submit">Save</button>
				<button type="button" class="secondary" on:click={cancel}>
					Cancel
				</button>
			</div>
		</form>
	</div>
</dialog>
