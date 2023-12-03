<script lang="ts">
	import { Page } from "$lib/page";
	import Settings from "$lib/component/Settings.svelte";
	import logo from "$lib/assets/Square71x71Logo.png";

	export let page: Page;
	export let lock: () => void | Promise<void>;
	export let visitChangeSettings: () => void | Promise<void>;
	export let visitChangePassword: () => void | Promise<void>;
</script>

{#if ![Page.Loading, Page.Unlock].includes(page)}
	<header class="container-fluid">
		<nav>
			<ul>
				<li>
					<img src={logo} alt="logo" />
				</li>
			</ul>
			<ul>
				<li>
					<Settings {lock} {visitChangeSettings} {visitChangePassword} />
				</li>
			</ul>
		</nav>
	</header>
{/if}
