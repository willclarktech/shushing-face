<script lang="ts">
	import logo from "$lib/assets/RectangleLogo.svg";
	import UnlockForm from "$lib/component/UnlockForm.svelte";

	export let alreadyExists: boolean;
	export let createPassword: (
		password: string,
		repeat: string
	) => void | Promise<void>;
	export let unlock: (password: string) => void | Promise<void>;
</script>

<div class="container-narrow">
	<div class="container-vertical">
		<img src={logo} alt="logo" />
		<UnlockForm {alreadyExists} {createPassword} {unlock} />
	</div>
</div>

<style>
	img {
		margin-bottom: 1rem;
	}
</style>
