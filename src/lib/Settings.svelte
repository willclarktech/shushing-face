<script lang="ts">
	export let lock: () => void | Promise<void>;
	let isOpen = false;

	const open = () => {
		isOpen = true;
	};

	const close = () => {
		isOpen = false;
	};
</script>

{#if isOpen}
	<button on:click={close}>⚙️</button>
	<ul>
		<li><button on:click={lock}>🔒</button></li>
	</ul>
{:else}
	<button on:click={open}>⚙️</button>
{/if}
