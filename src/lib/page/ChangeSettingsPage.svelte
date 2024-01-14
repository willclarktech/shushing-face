<script lang="ts">
	import type { Config } from "$lib/model";
	import ChangeSettingsForm from "$lib/component/ChangeSettingsForm.svelte";

	export let updateSettings: (config: Config) => void | Promise<void>;
	export let visitTasks: () => void | Promise<void>;

	export let config: Config;
</script>

<div class="container-narrow">
	<div class="container-vertical">
		<ChangeSettingsForm {updateSettings} {visitTasks} {config} />
	</div>
</div>
