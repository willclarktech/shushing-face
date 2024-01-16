<script lang="ts">
	import Iconify, { type IconifyIcon } from "@iconify/svelte";
	import cog from "@iconify/icons-clarity/cog-solid";
	import edit from "@iconify/icons-clarity/edit-solid";
	import eyeHide from "@iconify/icons-clarity/eye-hide-solid";
	import eyeShow from "@iconify/icons-clarity/eye-solid";
	import floppy from "@iconify/icons-clarity/floppy-solid";
	import lock from "@iconify/icons-clarity/lock-solid";
	import plus from "@iconify/icons-clarity/plus-circle-solid";
	import recycle from "@iconify/icons-clarity/recycle-solid";
	import sun from "@iconify/icons-clarity/sun-solid";
	import times from "@iconify/icons-clarity/times-line";
	import trash from "@iconify/icons-clarity/trash-solid";
	import twoWayArrows from "@iconify/icons-clarity/two-way-arrows-line";
	import unlock from "@iconify/icons-clarity/unlock-solid";
	import warning from "@iconify/icons-clarity/warning-solid";

	export let variant: string;

	const icons: Readonly<Record<string, IconifyIcon | undefined>> = {
		cog,
		edit,
		eyeHide,
		eyeShow,
		floppy,
		lock,
		plus,
		recycle,
		sun,
		times,
		trash,
		twoWayArrows,
		unlock,
		warning,
	};
	$: icon = icons[variant] ?? warning;
</script>

<Iconify inline {icon} />
