<script lang="ts">
	import Icon from "$lib/component/Icon.svelte";

	export let id: string;
	export let placeholder: string;
	export let value: string;
	export let invalid: boolean | null = null;
	export let errorMessage: string | null = null;
	export let onInput: undefined | (() => void | Promise<void>) = undefined;

	let inputElement: HTMLInputElement;
	let showPassword = false;

	const togglePasswordVisibility = () => {
		showPassword = !showPassword;
		// NOTE: Can't bind input type
		inputElement.type = showPassword ? "text" : "password";
	};

	$: variant = showPassword ? "eyeHide" : "eyeShow";
	$: errorId = `${id}-error`;
</script>

<div>
	{#if errorMessage}
		<small id={errorId} class="error-message">{errorMessage}</small>
	{/if}
	<div class="input-group">
		<input
			bind:this={inputElement}
			{id}
			name={id}
			type="password"
			{placeholder}
			bind:value
			autocomplete="off"
			autocapitalize="off"
			autocorrect="off"
			required
			on:input={onInput}
			aria-invalid={invalid}
			aria-errormessage={errorMessage ? errorId : undefined}
		/>
		<button
			type="button"
			class="toggle-visibility"
			on:click={togglePasswordVisibility}
		>
			<Icon {variant} />
		</button>
	</div>
</div>

<style>
	.input-group {
		display: flex;
	}

	.toggle-visibility {
		max-width: 3rem;
		right: 0;
		align-self: center;
	}

	.error-message {
		color: red;
	}
</style>
