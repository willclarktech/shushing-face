<script lang="ts">
	import Icon from "$lib/component/Icon.svelte";
	import { MINIMUM_PASSWORD_LENGTH } from "$lib/constant";
	import PasswordInput from "./PasswordInput.svelte";

	export let changePassword: (
		password: string,
		newPassword: string,
		repeatPassword: string
	) => void | Promise<void>;
	export let visitTasks: () => void | Promise<void>;

	let currentPasswordValue: string = "";
	let newPasswordValue: string = "";
	let repeatPasswordValue: string = "";

	const submit = async () => {
		try {
			await changePassword(
				currentPasswordValue,
				newPasswordValue,
				repeatPasswordValue
			);
			visitTasks();
		} catch (error) {
			console.error(error);
		}
	};

	let passwordTouched = false;
	let repeatTouched = false;
	const setPasswordTouched = () => {
		passwordTouched = true;
	};
	const setRepeatTouched = () => {
		repeatTouched = true;
	};

	$: newPasswordInvalid = passwordTouched
		? newPasswordValue.length < MINIMUM_PASSWORD_LENGTH
		: null;
	$: repeatPasswordInvalid = repeatTouched
		? repeatPasswordValue.length < MINIMUM_PASSWORD_LENGTH ||
		  repeatPasswordValue !== newPasswordValue
		: null;
</script>

<div class="container-narrow">
	<div class="container-vertical">
		<form on:submit|preventDefault={submit}>
			<fieldset>
				<PasswordInput
					id="current"
					placeholder="Current password"
					bind:value={currentPasswordValue}
				/>
				<PasswordInput
					id="new"
					placeholder="New password"
					bind:value={newPasswordValue}
					onInput={setPasswordTouched}
					invalid={newPasswordInvalid}
				/>
				<PasswordInput
					id="repeat"
					placeholder="Repeat password"
					bind:value={repeatPasswordValue}
					onInput={setRepeatTouched}
					invalid={repeatPasswordInvalid}
				/>
				<div class="grid">
					<button type="submit">
						<Icon variant="lock" />
						Change password
					</button>
					<button class="secondary" on:click={visitTasks}>
						<Icon variant="times" />
						Cancel
					</button>
				</div>
			</fieldset>
		</form>
	</div>
</div>
