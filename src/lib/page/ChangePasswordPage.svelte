<script lang="ts">
	import ChangePasswordForm from "$lib/component/ChangePasswordForm.svelte";

	export let changePassword: (
		currentPassword: string,
		newPassword: string,
		repeatPassword: string
	) => void | Promise<void>;
	export let visitTasks: () => void | Promise<void>;
</script>

<div class="container-narrow">
	<div class="container-vertical">
		<ChangePasswordForm {changePassword} {visitTasks} />
	</div>
</div>
