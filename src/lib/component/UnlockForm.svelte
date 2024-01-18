<script lang="ts">
	import { Form, createForm } from "svelte-forms-lib";
	import * as yup from "yup";

	import { MINIMUM_PASSWORD_LENGTH } from "$lib/model";
	import PasswordInput from "./PasswordInput.svelte";

	export let alreadyExists: boolean;
	export let createPassword: (
		password: string,
		repeatPassword: string
	) => void | Promise<void>;
	export let unlock: (password: string) => void | Promise<void>;

	interface FormValues {
		password: string;
		repeatPassword: string;
	}

	const initialValues: FormValues = {
		password: "",
		repeatPassword: "",
	};

	const validationSchema = yup
		.object({
			password: yup
				.string()
				.required("Password is required.")
				.min(
					alreadyExists ? 0 : MINIMUM_PASSWORD_LENGTH,
					`Password must be at least ${MINIMUM_PASSWORD_LENGTH} characters long.`
				),
			repeatPassword: alreadyExists
				? yup.string().notRequired()
				: yup
						.string()
						.required("Repeat password is required.")
						.oneOf([yup.ref("password")], "Passwords must match."),
		})
		.defined();

	const onSubmit = async (values: FormValues): Promise<void> => {
		try {
			if (alreadyExists) {
				await unlock(values.password);
			} else {
				await createPassword(values.password, values.repeatPassword);
			}
		} catch (error) {
			if (/aes\-gcm/i.test(error as string)) {
				context.errors.update((e) => ({
					...e,
					password: "Incorrect password",
				}));
			} else {
				// TODO: Make this a debug statement and handle
				console.error(error);
			}
		}
	};

	// HACK: `onSubmit` needs to access the context in order to validate the password field
	// and `svelte-forms-lib` does not export the type `FormState`
	var context = createForm({ initialValues, validationSchema, onSubmit });

	$: errors = context.errors;
	$: touched = context.touched;
	// NOTE: Do not show existing password as valid before it has been sent to the back end
	$: passwordInvalid = alreadyExists
		? $errors.password
			? true
			: null
		: $touched.password
		? !!$errors.password
		: null;
	$: repeatPasswordInvalid = $touched.repeatPassword
		? !!$errors.repeatPassword
		: null;
</script>

<Form {context}>
	<fieldset>
		<PasswordInput
			id="password"
			label="Enter password"
			placeholder="Enter password"
			invalid={passwordInvalid}
		/>
		{#if !alreadyExists}
			<PasswordInput
				id="repeatPassword"
				label="Repeat password"
				placeholder="Repeat password"
				invalid={repeatPasswordInvalid}
			/>
		{/if}
		<button type="submit">
			{#if alreadyExists}
				Unlock
			{:else}
				Set password
			{/if}
		</button>
	</fieldset>
</Form>
