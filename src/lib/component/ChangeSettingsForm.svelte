<script lang="ts">
	import type { Config } from "$lib/config";
	import type { ChangeEventHandler } from "svelte/elements";
	import { MILLISECONDS_PER_MINUTE } from "$lib/constant";

	export let updateSettings: (config: Config) => void | Promise<void>;
	export let visitTasks: () => void | Promise<void>;

	export let config: Config;
	let newConfig = config;

	const updateAutoLockTimeout: ChangeEventHandler<HTMLInputElement> = (
		event
	): void => {
		const newValue = parseInt(event.currentTarget.value, 10);
		if (!isNaN(newValue)) {
			newConfig.autoLockTimeout = newValue * MILLISECONDS_PER_MINUTE;
		}
	};

	const submit = async () => {
		try {
			await updateSettings(newConfig);
			visitTasks();
		} catch (error) {
			console.error(error);
		}
	};

	$: autoLockTimeoutMinutes =
		newConfig.autoLockTimeout / MILLISECONDS_PER_MINUTE;
</script>

<div class="container-narrow">
	<div class="container-vertical">
		<form on:submit|preventDefault={submit}>
			<fieldset>
				<label for="autoLockTimeout">Auto-lock timeout (minutes)</label>
				<input
					id="autoLockTimeout"
					name="autoLockTimeout"
					type="number"
					on:change={updateAutoLockTimeout}
					value={autoLockTimeoutMinutes}
					step="1"
					min="1"
					max="1440"
				/>
				<div class="grid">
					<button type="submit"> Update </button>
					<button class="secondary" on:click={visitTasks}> Cancel </button>
				</div>
			</fieldset>
		</form>
	</div>
</div>
