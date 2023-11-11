<script lang="ts">
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

<button on:click={open}>⚙️</button>
<dialog class="dialog-settings" open={isOpen}>
	<div class="container-narrow">
		<button
			on:click={() => {
				visitChangePassword();
				close();
			}}
		>
			Change password
		</button>
		<button on:click={lock}>Lock 🔒</button>
		<button on:click={close}>Cancel ❌</button>
	</div>
</dialog>

<style>
	.dialog-settings button {
		margin: 10px 0px;
	}
</style>
