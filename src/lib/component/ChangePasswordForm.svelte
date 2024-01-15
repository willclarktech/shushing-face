<script lang="ts">
	import { MINIMUM_PASSWORD_LENGTH } from "$lib/model";
	import PasswordInput from "./PasswordInput.svelte";

	export let changePassword: (
		currentPassword: string,
		newPassword: string,
		repeatPassword: string
	) => void | Promise<void>;
	export let visitTasks: () => void | Promise<void>;

	let currentPasswordValue: string = "";
	let newPasswordValue: string = "";
	let repeatPasswordValue: string = "";
	let submissionAttempted = false;
	let errorMessage: string | null = null;
	let currentPasswordError: string | null = null;
	let newPasswordError: string | null = null;
	let repeatPasswordError: string | null = null;

	const incorrectPasswordMessage = "Incorrect password";

	const clearValidation = () => {
		submissionAttempted = false;
		currentPasswordError = null;
		newPasswordError = null;
		repeatPasswordError = null;
		errorMessage = null;
	};

	const validateForm = () => {
		let isValid = true;

		if (currentPasswordValue.length === 0) {
			currentPasswordError = "Current password is required.";
			isValid = false;
		} else {
			currentPasswordError = null;
		}

		if (newPasswordValue.length < MINIMUM_PASSWORD_LENGTH) {
			newPasswordError = `New password must be at least ${MINIMUM_PASSWORD_LENGTH} characters long.`;
			isValid = false;
		} else if (newPasswordValue === currentPasswordValue) {
			newPasswordError =
				"New password must not be the same as current password.";
			isValid = false;
		} else {
			newPasswordError = null;
		}

		if (repeatPasswordValue !== newPasswordValue) {
			repeatPasswordError = "Passwords do not match.";
			isValid = false;
		} else {
			repeatPasswordError = null;
		}

		return isValid;
	};

	const submit = async () => {
		submissionAttempted = true;
		const isFormValid = validateForm();
		if (!isFormValid) {
			return;
		}

		try {
			await changePassword(
				currentPasswordValue,
				newPasswordValue,
				repeatPasswordValue
			);
			visitTasks();
		} catch (error) {
			if (/incorrect password/i.test(error as string)) {
				currentPasswordError = incorrectPasswordMessage;
			} else {
				errorMessage = "Failed to change password. Please try again.";
			}
		}
	};
</script>

<div class="container-narrow">
	<div class="container-vertical">
		<form on:submit|preventDefault={submit}>
			<fieldset>
				<PasswordInput
					id="current"
					placeholder="Current password"
					bind:value={currentPasswordValue}
					invalid={typeof currentPasswordError === "string"
						? true
						: currentPasswordError}
					errorMessage={currentPasswordError}
					onInput={clearValidation}
				/>
				<PasswordInput
					id="new"
					placeholder="New password"
					bind:value={newPasswordValue}
					invalid={typeof newPasswordError === "string"
						? true
						: newPasswordError}
					errorMessage={newPasswordError}
					onInput={clearValidation}
				/>
				<PasswordInput
					id="repeat"
					placeholder="Repeat password"
					bind:value={repeatPasswordValue}
					invalid={typeof repeatPasswordError === "string"
						? true
						: repeatPasswordError}
					errorMessage={repeatPasswordError}
					onInput={clearValidation}
				/>
				{#if submissionAttempted && errorMessage}
					<small class="error-message">{errorMessage}</small>
				{/if}
				<div class="grid">
					<button type="submit"> Change password </button>
					<button class="secondary" on:click={visitTasks}> Cancel </button>
				</div>
			</fieldset>
		</form>
	</div>
</div>

<style>
	.error-message {
		color: red;
	}
</style>
