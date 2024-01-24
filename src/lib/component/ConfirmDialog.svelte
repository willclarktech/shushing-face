<script lang="ts">
	export let message = "";
	export let onConfirm: () => void = () => {};
	export let onCancel: () => void = () => {};
	export let isOpen = false;
</script>

<dialog open={isOpen}>
	<article>
		<p>{message}</p>
		<footer>
			<button type="button" on:click={onConfirm}>Confirm</button>
			<button type="button" class="secondary" on:click={onCancel}>
				Cancel
			</button>
		</footer>
	</article>
</dialog>
