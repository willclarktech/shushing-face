<script lang="ts">
	import logo from "$lib/assets/Square71x71Logo.png";
	import Icon from "$lib/component/Icon.svelte";

	export let lock: () => void | Promise<void>;
	export let visitHome: () => void | Promise<void>;
	export let visitChangeSettings: () => void | Promise<void>;
	export let visitChangePassword: () => void | Promise<void>;
</script>

<header class="container-fluid">
	<nav>
		<ul>
			<li>
				<button class="logo-home" on:click={visitHome}
					><img src={logo} alt="logo" /></button
				>
			</li>
		</ul>
		<ul>
			<li>
				<button
					type="button"
					on:click={() => {
						visitChangePassword();
						close();
					}}
				>
					<Icon variant="key" />
				</button>
				<button
					type="button"
					on:click={() => {
						visitChangeSettings();
						close();
					}}
				>
					<Icon variant="cog" />
				</button>
				<button type="button" on:click={lock}><Icon variant="lock" /></button>
			</li>
		</ul>
	</nav>
</header>

<style>
	.logo-home {
		background: transparent;
		border: none;
	}
</style>
