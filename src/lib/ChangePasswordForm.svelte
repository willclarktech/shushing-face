<script lang="ts">
	import Icon from "./Icon.svelte";
	import { MINIMUM_PASSWORD_LENGTH } from "./constant";

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
</script>

<div class="container-narrow">
	<div class="container-vertical">
		<form on:submit|preventDefault={submit}>
			<fieldset>
				<input
					id="current"
					name="current"
					type="password"
					placeholder="Current password"
					autocomplete={"off"}
					autocapitalize={"off"}
					autocorrect={"off"}
					bind:value={currentPasswordValue}
					required
				/>
				<input
					id="new"
					name="new"
					type="password"
					placeholder="New password"
					autocomplete={"off"}
					autocapitalize={"off"}
					autocorrect={"off"}
					on:input={setPasswordTouched}
					aria-invalid={passwordTouched
						? newPasswordValue.length < MINIMUM_PASSWORD_LENGTH
						: null}
					bind:value={newPasswordValue}
					required
				/>
				<input
					id="repeat"
					name="repeat"
					type="password"
					placeholder="Repeat password"
					autocomplete={"off"}
					autocapitalize={"off"}
					autocorrect={"off"}
					on:input={setRepeatTouched}
					aria-invalid={repeatTouched
						? repeatPasswordValue.length < MINIMUM_PASSWORD_LENGTH ||
						  repeatPasswordValue !== newPasswordValue
						: null}
					bind:value={repeatPasswordValue}
					required
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
