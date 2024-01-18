<script lang="ts">
	import type { Config } from "$lib/model";
	import ChangeSettingsForm from "$lib/component/ChangeSettingsForm.svelte";

	export let updateSettings: (config: Config) => void | Promise<void>;
	export let onDone: () => void | Promise<void>;
	export let config: Config;
</script>

<main class="container">
	<section>
		<h1>Settings</h1>
		<ChangeSettingsForm {updateSettings} {onDone} {config} />
	</section>
</main>
