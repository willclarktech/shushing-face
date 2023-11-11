<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	import type { Task } from "./types";
	import ChangePasswordForm from "./ChangePasswordForm.svelte";
	import Settings from "./Settings.svelte";
	import Tasks from "./Tasks.svelte";
	import UnlockForm from "./UnlockForm.svelte";
	import { onMount } from "svelte";

	enum Page {
		Loading,
		Unlock,
		Tasks,
		ChangePassword,
	}

	let alreadyExists = false;
	let page = Page.Loading;
	let tasks: Task[] = [];

	const unlock = async (password: string) => {
		await invoke("unlock", { password });
		try {
			tasks = await invoke("load_tasks");
			page = Page.Tasks;
			alreadyExists = true;
		} catch (error) {
			console.log(`error: ${error}`);
		}
	};

	const lock = async () => {
		await invoke("lock");
		tasks = [];
		page = Page.Unlock;
	};

	const createTask = (
		id: number,
		description: string,
		deadline: string,
		completed = false
	): Task => {
		const trimmedDescription = description.trim();
		const deadlineNumber = new Date(deadline).getTime();

		if (trimmedDescription.length === 0 || isNaN(deadlineNumber)) {
			throw new Error("Invalid task info");
		}
		return {
			id,
			description: trimmedDescription,
			deadline: deadlineNumber,
			completed,
		};
	};

	const addTask = async (description: string, deadline: string) => {
		const task = createTask(Date.now(), description, deadline);
		tasks = [...tasks, task].sort((a, b) => a.deadline - b.deadline);
		await invoke("save_tasks", { tasks });
	};

	const editTask = async (
		taskId: number,
		description: string,
		deadline: string
	) => {
		tasks = tasks
			.map((task) => {
				if (task.id !== taskId) {
					return task;
				}
				return createTask(taskId, description, deadline, task.completed);
			})
			.sort((a, b) => a.deadline - b.deadline);
		await invoke("save_tasks", { tasks });
	};

	const toggleComplete = async (taskId: number) => {
		tasks = tasks.map((task) => {
			if (task.id !== taskId) {
				return task;
			}
			return { ...task, completed: !task.completed };
		});
		await invoke("save_tasks", { tasks });
	};

	const deleteTask = async (taskId: number) => {
		tasks = tasks.filter((task) => task.id !== taskId);
		await invoke("save_tasks", { tasks });
	};

	const changePassword = async (
		currentPassword: string,
		newPassword: string,
		repeatPassword: string
	) => {
		if (newPassword !== repeatPassword) {
			throw new Error("Passwords don’t match");
		}
		await invoke("change_password", {
			current: currentPassword,
			new: newPassword,
		});
	};

	const visitChangePassword = () => {
		page = Page.ChangePassword;
	};

	onMount(async () => {
		alreadyExists = await invoke("check_exists");
		page = Page.Unlock;
	});
</script>

{#if [Page.Tasks, Page.ChangePassword].includes(page)}
	<Settings {lock} {visitChangePassword} />
{/if}

{#if page === Page.Loading}
	Loading...
{:else if page === Page.Unlock}
	<UnlockForm {alreadyExists} {unlock} />
{:else if page === Page.Tasks}
	<Tasks {tasks} {addTask} {editTask} {toggleComplete} {deleteTask} />
{:else if page === Page.ChangePassword}
	<ChangePasswordForm {changePassword} />
{/if}
