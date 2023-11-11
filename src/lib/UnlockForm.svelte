<script lang="ts">
	import logo from "./assets/RectangleLogo.svg";

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
						bind:value={repeatValue}
						required
					/>
				{/if}
				<button type="submit"
					>{alreadyExists ? "🔓 Unlock" : "🆕 Set password"}</button
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
