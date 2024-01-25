<script lang="ts">
	import { ErrorMessage, Field } from "svelte-forms-lib";

	import Icon from "$lib/component/Icon.svelte";

	export let id: string;
	export let label: string;
	export let placeholder: string = "";
	export let invalid: boolean | null = null;
	export let helpText: string = "";

	let showPassword = false;

	const togglePasswordVisibility = () => {
		showPassword = !showPassword;
	};

	$: inputType = showPassword ? "text" : "password";
	$: variant = showPassword ? "eyeHide" : "eyeShow";
</script>

<div>
	<div class="grid" aria-invalid={invalid}>
		<!-- See https://github.com/picocss/pico/issues/430 -->
		<!-- svelte-ignore a11y-no-redundant-roles -->
		<fieldset role="group">
			<Field
				{id}
				name={id}
				type={inputType}
				{label}
				{placeholder}
				autocomplete="off"
				autocapitalize="off"
				autocorrect="off"
				required
				aria-invalid={invalid}
			/>
			<button type="button" on:click={togglePasswordVisibility}>
				<Icon {variant} />
			</button>
		</fieldset>
	</div>
	<ErrorMessage name={id} />
	{#if helpText}
		<small>{helpText}</small>
	{/if}
</div>
