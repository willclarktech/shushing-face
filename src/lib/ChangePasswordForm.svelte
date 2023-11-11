<script lang="ts">
	export let changePassword: (
		password: string,
		newPassword: string,
		repeatPassword: string
	) => void | Promise<void>;

	let currentPasswordValue: string = "";
	let newPasswordValue: string = "";
	let repeatPasswordValue: string = "";

	const submit = () => {
		changePassword(currentPasswordValue, newPasswordValue, repeatPasswordValue);
	};
</script>

<label>
	Current password:
	<input
		type="text"
		autocomplete={"off"}
		autocapitalize={"off"}
		autocorrect={"off"}
		bind:value={currentPasswordValue}
	/>
</label>
<label>
	New password:
	<input
		type="text"
		autocomplete={"off"}
		autocapitalize={"off"}
		autocorrect={"off"}
		bind:value={newPasswordValue}
	/>
</label>
<label>
	Repeat password:
	<input
		type="text"
		autocomplete={"off"}
		autocapitalize={"off"}
		autocorrect={"off"}
		bind:value={repeatPasswordValue}
	/>
</label>
<button on:click={submit}>👍</button>
