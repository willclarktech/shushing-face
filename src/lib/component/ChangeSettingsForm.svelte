<script lang="ts">
	import type { Config } from "$lib/model";
	import { Field, Form, createForm } from "svelte-forms-lib";

	export let updateSettings: (config: Config) => void | Promise<void>;
	export let onDone: () => void | Promise<void>;

	export let config: Config;

	type FormValues = Omit<Config, "autoLockTimeout"> & {
		autoLockTimeout: string;
	};

	const initialValues: FormValues = {
		...config,
		autoLockTimeout: config.autoLockTimeout.toString(10),
	};

	const onSubmit = async (values: FormValues) => {
		try {
			await updateSettings({
				...values,
				autoLockTimeout: parseInt(values.autoLockTimeout, 10),
			});
			onDone();
		} catch (error) {
			// TODO: Make this a debug statement and handle
			console.error(error);
		}
	};

	const context = createForm({
		initialValues,
		onSubmit,
	});

	$: form = context.form;
</script>

<Form {context}>
	<fieldset>
		<label for="autoLockTimeout">
			Auto-lock timeout (minutes)
			<Field
				id="autoLockTimeout"
				name="autoLockTimeout"
				type="number"
				step="1"
				min="1"
				max="1440"
			/>
		</label>
	</fieldset>
	<fieldset>
		<h2>Cloud Storage</h2>
		<label for="icloudEnabled">
			<Field
				id="icloudEnabled"
				name="icloudEnabled"
				type="checkbox"
				checked={$form.icloudEnabled}
			/> Enable iCloud
		</label>
		<label for="dropboxEnabled">
			<Field
				id="dropboxEnabled"
				name="dropboxEnabled"
				type="checkbox"
				checked={$form.dropboxEnabled}
			/> Enable Dropbox
		</label>
	</fieldset>
	<div class="grid">
		<button type="submit">Update</button>
		<button type="button" class="secondary" on:click={onDone}>Cancel</button>
	</div>
</Form>
