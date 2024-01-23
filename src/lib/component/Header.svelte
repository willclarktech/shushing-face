<script lang="ts">
	import Icon from "$lib/component/Icon.svelte";
	import Logo from "$lib/component/Logo.svelte";

	export let visitHome: () => void | Promise<void>;
	export let visitChangeSettings: () => void | Promise<void>;
	export let visitChangePassword: () => void | Promise<void>;
	export let openTaskForm: () => void | Promise<void>;
	export let lock: () => void | Promise<void>;
</script>

<header class="container-fluid">
	<nav>
		<ul>
			<li>
				<button type="button" class="logo-home" on:click={visitHome}>
					<Logo />
				</button>
			</li>
		</ul>
		<ul>
			<li>
				<button type="button" on:click={openTaskForm}> Add task </button>
				<button type="button" on:click={visitChangePassword}>
					<Icon variant="key" />
				</button>
				<button type="button" on:click={visitChangeSettings}>
					<Icon variant="cog" />
				</button>
				<button type="button" class="danger" on:click={lock}>
					<Icon variant="lock" />
				</button>
			</li>
		</ul>
	</nav>
</header>

<style>
	.logo-home {
		background: transparent;
		border: none;
		padding: 0;
	}

	.logo-home:has(> span:only-child) {
		font-size: 2.5rem;
	}
</style>
