<script lang="ts">
	import UnlockForm from "$lib/component/UnlockForm.svelte";
	import LogoWithText from "$lib/component/LogoWithText.svelte";

	export let alreadyExists: boolean;
	export let unlock: (password: string) => void | Promise<void>;
</script>

<section class="container-narrow container-vertical">
	<LogoWithText />
	<UnlockForm {alreadyExists} {unlock} />
</section>

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
