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

<main class="container-narrow container-vertical">
	<section>
		<img src={logo} alt="logo" />
	</section>
	<section>
		<UnlockForm {alreadyExists} {createPassword} {unlock} />
	</section>
</main>

<style>
	.container-narrow {
		margin: 0 auto;
		max-width: 20rem;
	}

	.container-vertical {
		height: 100vh;
		display: flex;
		flex-direction: column;
		justify-content: center;
	}
</style>
