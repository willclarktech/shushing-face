<script lang="ts">
	import { onDestroy, onMount } from "svelte";

	export let isUnlocked: boolean;
	export let timeout: number;
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

	// Clean up the timer and event listeners
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

	$: if (isUnlocked) {
		resetAutoLockTimer();
	} else {
		clearTimeout(autoLockTimer);
	}
</script>
