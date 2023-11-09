<script lang="ts">
	export let unlock: (password: string) => void | Promise<void>;
	let passwordValue: string = "";

	const submit = () => {
		unlock(passwordValue);
	};
</script>

<label>
	Password:
	<input type="text" bind:value={passwordValue} />
</label>
<button on:click={submit}>Unlock</button>
