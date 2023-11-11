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

<!-- <a href="javascript:void(0)" role="button" on:click={isOpen ? close : open}>
	⚙️
</a>
{#if isOpen}
	<ul>
		<li><button on:click={visitChangePassword}>Change password</button></li>
		<li><button on:click={lock}>🔒</button></li>
	</ul>
{/if} -->
<!--
<details role="list">
	<summary aria-haspopup="listbox">⚙️</summary>
	<ul role="listbox">
		<li><button on:click={visitChangePassword}>Change password</button></li>
		<li><button on:click={lock}>🔒</button></li>
	</ul>
</details> -->

<!-- <a href="javascript:void(0)" role="button" on:click={isOpen ? close : open} /> -->
<button on:click={open}>⚙️</button>
<dialog class="dialog-settings" open={isOpen}>
	<article>
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
	</article>
</dialog>

<style>
	.dialog-settings {
		min-width: 100vw;
	}
</style>
