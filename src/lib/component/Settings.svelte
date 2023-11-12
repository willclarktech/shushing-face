<script lang="ts">
	import Icon from "$lib/component/Icon.svelte";

	export let lock: () => void | Promise<void>;
	export let visitChangePassword: () => void | Promise<void>;

	let isOpen = false;

	const open = () => {
		isOpen = true;
	};
	const close = () => {
		isOpen = false;
	};
</script>

<button on:click={open}><Icon variant="cog" /></button>
<dialog class="dialog-settings" open={isOpen}>
	<div class="container-narrow">
		<button
			on:click={() => {
				visitChangePassword();
				close();
			}}
		>
			<Icon variant="twoWayArrows" />
			Change password
		</button>
		<button on:click={lock}><Icon variant="lock" /> Lock</button>
		<button on:click={close} class="secondary">
			<Icon variant="times" /> Cancel
		</button>
	</div>
</dialog>

<style>
	.dialog-settings button {
		margin: 10px 0px;
	}
</style>
