<script lang="ts">
	import ChangePasswordForm from "$lib/component/ChangePasswordForm.svelte";

	export let changePassword: (
		currentPassword: string,
		newPassword: string,
		repeatPassword: string
	) => void | Promise<void>;
	export let onDone: () => void | Promise<void>;
</script>

<main class="container">
	<section>
		<h1>Change Password</h1>
		<ChangePasswordForm {changePassword} {onDone} />
	</section>
</main>
