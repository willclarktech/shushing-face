<script lang="ts">
	export let alreadyExists: boolean;
	export let unlock: (password: string) => void | Promise<void>;
	let passwordValue: string = "";

	const submit = () => {
		unlock(passwordValue);
	};
</script>

<label>
	Password:
	<input
		type="text"
		autocomplete={"off"}
		autocapitalize={"off"}
		autocorrect={"off"}
		bind:value={passwordValue}
	/>
</label>
<button on:click={submit}
	>{alreadyExists ? "Unlock" : "Create new password"}</button
>
