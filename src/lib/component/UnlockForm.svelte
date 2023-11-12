<script lang="ts">
	import Icon from "$lib/component/Icon.svelte";
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
</script>

<div class="container-narrow">
	<div class="container-vertical">
		<img src={logo} alt="logo" />
		<form on:submit|preventDefault={submit}>
			<fieldset>
				<!-- <label for="password">Password:</label> -->
				<input
					id="password"
					name="password"
					type="password"
					placeholder="Enter password"
					autocomplete="off"
					autocapitalize="off"
					autocorrect="off"
					on:input={setPasswordTouched}
					aria-invalid={!alreadyExists && passwordTouched
						? passwordValue.length < MINIMUM_PASSWORD_LENGTH
						: null}
					bind:value={passwordValue}
					required
				/>
				{#if !alreadyExists}
					<!-- <label for="repeat">Repeat:</label> -->
					<input
						id="repeat"
						name="repeat"
						type="password"
						placeholder="Repeat password"
						autocomplete="off"
						autocapitalize="off"
						autocorrect="off"
						on:input={setRepeatTouched}
						aria-invalid={repeatTouched
							? repeatValue.length < MINIMUM_PASSWORD_LENGTH ||
							  repeatValue !== passwordValue
							: null}
						bind:value={repeatValue}
						required
					/>
				{/if}
				<button type="submit">
					{#if alreadyExists}
						<Icon variant="lock" />
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
