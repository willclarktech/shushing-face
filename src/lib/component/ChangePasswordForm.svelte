<script lang="ts">
	import { createForm } from "svelte-forms-lib";
	import * as yup from "yup";

	import { MINIMUM_PASSWORD_LENGTH } from "$lib/model";
	import PasswordInput from "./PasswordInput.svelte";

	export let changePassword: (
		currentPassword: string,
		newPassword: string,
		repeatPassword: string
	) => void | Promise<void>;
	export let visitTasks: () => void | Promise<void>;

	let submissionError: string | null = null;

	const validationSchema = yup.object({
		currentPassword: yup.string().required("Current password is required."),
		newPassword: yup
			.string()
			.min(
				MINIMUM_PASSWORD_LENGTH,
				`New password must be at least ${MINIMUM_PASSWORD_LENGTH} characters long.`
			)
			.notOneOf(
				[yup.ref("currentPassword")],
				"New password must not be the same as current password."
			),
		repeatPassword: yup
			.string()
			.oneOf([yup.ref("newPassword")], "Passwords do not match."),
	});

	const { form, errors, handleSubmit } = createForm({
		initialValues: {
			currentPassword: "",
			newPassword: "",
			repeatPassword: "",
		},
		validationSchema,
		onSubmit: async (values) => {
			try {
				await changePassword(
					values.currentPassword,
					values.newPassword,
					values.repeatPassword
				);
				visitTasks();
			} catch (error) {
				if (/incorrect password/i.test(error as string)) {
					errors.update((e) => ({
						...e,
						currentPassword: "Incorrect password.",
					}));
				} else {
					submissionError = "Failed to change password. Please try again.";
				}
			}
		},
	});
</script>

<div class="container-narrow">
	<div class="container-vertical">
		<form on:submit|preventDefault={handleSubmit}>
			<fieldset>
				<PasswordInput
					id="current"
					placeholder="Current password"
					bind:value={$form.currentPassword}
					invalid={!!$errors.currentPassword}
					errorMessage={$errors.currentPassword}
				/>
				<PasswordInput
					id="new"
					placeholder="New password"
					bind:value={$form.newPassword}
					invalid={!!$errors.newPassword}
					errorMessage={$errors.newPassword}
				/>
				<PasswordInput
					id="repeat"
					placeholder="Repeat password"
					bind:value={$form.repeatPassword}
					invalid={!!$errors.repeatPassword}
					errorMessage={$errors.repeatPassword}
				/>
				{#if submissionError}
					<small class="error-message">{submissionError}</small>
				{/if}
				<div class="grid">
					<button type="submit">Change password</button>
					<button class="secondary" on:click={visitTasks}>Cancel</button>
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
