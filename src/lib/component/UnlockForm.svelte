<script lang="ts">
	import Icon from "$lib/component/Icon.svelte";
	import PasswordInput from "$lib/component/PasswordInput.svelte";
	import { MINIMUM_PASSWORD_LENGTH } from "$lib/constant";
	import logo from "$lib/assets/RectangleLogo.svg";

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

	let passwordTouched = false;
	let repeatTouched = false;
	const setPasswordTouched = () => {
		passwordTouched = true;
	};
	const setRepeatTouched = () => {
		repeatTouched = true;
	};

	$: passwordInvalid =
		!alreadyExists && passwordTouched
			? passwordValue.length < MINIMUM_PASSWORD_LENGTH
			: null;
	$: repeatInvalid = repeatTouched
		? repeatValue.length < MINIMUM_PASSWORD_LENGTH ||
		  repeatValue !== passwordValue
		: null;
</script>

<div class="container-narrow">
	<div class="container-vertical">
		<img src={logo} alt="logo" />
		<form on:submit|preventDefault={submit}>
			<fieldset>
				<PasswordInput
					id="password"
					placeholder="Enter password"
					bind:value={passwordValue}
					onInput={setPasswordTouched}
					invalid={passwordInvalid}
				/>
				{#if !alreadyExists}
					<PasswordInput
						id="repeat"
						placeholder="Repeat password"
						bind:value={repeatValue}
						onInput={setRepeatTouched}
						invalid={repeatInvalid}
					/>
				{/if}
				<button type="submit">
					{#if alreadyExists}
						<Icon variant="unlock" />
						Unlock
					{:else}
						<Icon variant="plus" />
						Set password
					{/if}</button
				>
			</fieldset>
		</form>
	</div>
</div>

<style>
	img {
		margin-bottom: 1rem;
	}
</style>
