<script lang="ts">
	import { Form, createForm } from "svelte-forms-lib";
	import * as yup from "yup";

	import { MINIMUM_PASSWORD_LENGTH } from "$lib/model";
	import PasswordInput from "./PasswordInput.svelte";

	export let changePassword: (
		currentPassword: string,
		newPassword: string,
		repeatPassword: string
	) => void | Promise<void>;
	export let onDone: () => void | Promise<void>;

	interface FormValues {
		currentPassword: string;
		newPassword: string;
		repeatPassword: string;
	}

	const initialValues: FormValues = {
		currentPassword: "",
		newPassword: "",
		repeatPassword: "",
	};

	const validationSchema: yup.ObjectSchema<FormValues> = yup.object({
		currentPassword: yup.string().required("Current password is required."),
		newPassword: yup
			.string()
			.required("New password is required.")
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
			.required("Repeat password is required.")
			.oneOf([yup.ref("newPassword")], "Passwords must match."),
	});

	const onSubmit = async (values: FormValues): Promise<void> => {
		try {
			await changePassword(
				values.currentPassword,
				values.newPassword,
				values.repeatPassword
			);
			onDone();
		} catch (error) {
			if (/incorrect password/i.test(error as string)) {
				context.errors.update((e) => ({
					...e,
					currentPassword: "Incorrect password.",
				}));
			} else {
				// TODO: Make this a debug statement and handle
				console.error(error);
			}
		}
	};

	var context = createForm({
		initialValues,
		validationSchema,
		onSubmit,
	});

	$: errors = context.errors;
	$: touched = context.touched;
	// NOTE: Do not show existing password as valid before it has been sent to the back end
	$: currentPasswordInvalid = $errors.currentPassword ? true : null;
	$: newPasswordInvalid = $touched.newPassword ? !!$errors.newPassword : null;
	$: repeatPasswordInvalid = $touched.repeatPassword
		? !!$errors.repeatPassword
		: null;
</script>

<Form {context}>
	<fieldset>
		<PasswordInput
			id="currentPassword"
			label="Current password"
			placeholder="Current password"
			invalid={currentPasswordInvalid}
		/>
	</fieldset>
	<fieldset>
		<PasswordInput
			id="newPassword"
			label="New password"
			placeholder="New password"
			invalid={newPasswordInvalid}
		/>
		<PasswordInput
			id="repeatPassword"
			label="Repeat password"
			placeholder="Repeat password"
			invalid={repeatPasswordInvalid}
		/>
	</fieldset>
	<div class="grid">
		<button type="submit">Change</button>
		<button type="button" class="secondary" on:click={onDone}>Cancel</button>
	</div>
</Form>
