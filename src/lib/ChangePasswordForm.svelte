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

<div class="container-narrow">
	<div class="container-vertical">
		<form on:submit|preventDefault={submit}>
			<fieldset>
				<!-- <label for="current">Current password:</label> -->
				<input
					id="current"
					name="current"
					type="password"
					placeholder="Current password"
					autocomplete={"off"}
					autocapitalize={"off"}
					autocorrect={"off"}
					bind:value={currentPasswordValue}
				/>
				<!-- <label for="new">New password:</label> -->
				<input
					id="new"
					name="new"
					type="password"
					placeholder="New password"
					autocomplete={"off"}
					autocapitalize={"off"}
					autocorrect={"off"}
					bind:value={newPasswordValue}
				/>
				<!-- <label for="new">Repeat password:</label> -->
				<input
					id="repeat"
					name="repeat"
					type="password"
					placeholder="Repeat password"
					autocomplete={"off"}
					autocapitalize={"off"}
					autocorrect={"off"}
					bind:value={repeatPasswordValue}
				/>
				<button type="submit">👍</button>
			</fieldset>
		</form>
	</div>
</div>
