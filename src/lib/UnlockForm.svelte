<script lang="ts">
	export let alreadyExists: boolean;
	export let createPassword: (
		password: string,
		repeat: string
	) => void | Promise<void>;
	export let unlock: (password: string) => void | Promise<void>;

	let passwordValue: string = "";
	let repeatValue: string = "";

	const submit = () => {
		if (alreadyExists) {
			unlock(passwordValue);
		} else {
			createPassword(passwordValue, repeatValue);
		}
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
	{#if !alreadyExists}
		<br />
		Repeat:
		<input
			type="text"
			autocomplete={"off"}
			autocapitalize={"off"}
			autocorrect={"off"}
			bind:value={repeatValue}
		/>
	{/if}
</label>
<br />
<button on:click={submit}>{alreadyExists ? "🔓" : "🆕"}</button>
