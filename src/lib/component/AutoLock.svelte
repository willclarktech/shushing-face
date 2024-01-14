<script lang="ts">
	import { onDestroy, onMount } from "svelte";
	import { DEFAULT_AUTO_LOCK_TIMEOUT } from "$lib/model";

	export let isUnlocked: boolean;
	export let timeout: number = DEFAULT_AUTO_LOCK_TIMEOUT;
	export let lock: () => void | Promise<void>;

	const activities = ["mousemove", "keypress", "click", "touchstart"];
	let autoLockTimer: ReturnType<typeof setTimeout>;

	const resetAutoLockTimer = () => {
		if (isUnlocked) {
			clearTimeout(autoLockTimer);
			autoLockTimer = setTimeout(lock, timeout);
		}
	};

	const setupActivityListeners = () => {
		activities.forEach((event) => {
			window.addEventListener(event, resetAutoLockTimer, { passive: true });
		});
	};

	const cleanupActivityListeners = () => {
		activities.forEach((event) => {
			window.removeEventListener(event, resetAutoLockTimer);
		});
		clearTimeout(autoLockTimer);
	};

	onMount(() => {
		setupActivityListeners();
		resetAutoLockTimer();
	});

	onDestroy(cleanupActivityListeners);

	$: if (isUnlocked && !isNaN(timeout)) {
		resetAutoLockTimer();
	} else {
		clearTimeout(autoLockTimer);
	}
</script>
