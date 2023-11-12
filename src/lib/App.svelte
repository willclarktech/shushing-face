<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";

	import { Page, type Task } from "$lib/types";
	import AutoLock from "$lib/component/AutoLock.svelte";
	import ChangePasswordForm from "$lib/component/ChangePasswordForm.svelte";
	import Header from "$lib/component/Header.svelte";
	import UnlockForm from "$lib/component/UnlockForm.svelte";
	import Tasks from "$lib/page/Tasks.svelte";
	import { AUTO_LOCK_TIMEOUT } from "$lib/constant";

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
			console.error(`error: ${error}`);
		}
	};

	const createPassword = async (password: string, repeat: string) => {
		if (password !== repeat) {
			throw new Error("Passwords don’t match");
		}
		await unlock(password);
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
		details: string,
		completed = false
	): Task => {
		const trimmedDescription = description.trim();
		const deadlineNumber = new Date(deadline).getTime();
		if (trimmedDescription.length === 0 || isNaN(deadlineNumber)) {
			throw new Error("Invalid task info");
		}

		const trimmedDetails = details.trim();
		return {
			id,
			description: trimmedDescription,
			details: trimmedDetails,
			deadline: deadlineNumber,
			completed,
		};
	};

	const addTask = async (
		description: string,
		deadline: string,
		details: string
	) => {
		const task = createTask(Date.now(), description, deadline, details);
		tasks = [...tasks, task].sort((a, b) => a.deadline - b.deadline);
		await invoke("save_tasks", { tasks });
	};

	const editTask = async (
		taskId: number,
		description: string,
		deadline: string,
		details: string
	) => {
		tasks = tasks
			.map((task) => {
				if (task.id !== taskId) {
					return task;
				}
				return createTask(
					taskId,
					description,
					deadline,
					details,
					task.completed
				);
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

	const visit = (pageToVisit: Page) => {
		page = pageToVisit;
	};

	const visitChangePassword = visit.bind(null, Page.ChangePassword);
	const visitTasks = visit.bind(null, Page.Tasks);

	onMount(async () => {
		alreadyExists = await invoke("check_exists");
		page = Page.Unlock;
	});
</script>

<Header {page} {lock} {visitChangePassword} />

<main>
	{#if page === Page.Loading}
		Loading...
	{:else if page === Page.Unlock}
		<UnlockForm {alreadyExists} {createPassword} {unlock} />
	{:else if page === Page.Tasks}
		<Tasks {tasks} {addTask} {editTask} {toggleComplete} {deleteTask} />
	{:else if page === Page.ChangePassword}
		<ChangePasswordForm {changePassword} {visitTasks} />
	{/if}
</main>

<footer>
	<AutoLock
		isUnlocked={![Page.Loading, Page.Unlock].includes(page)}
		{lock}
		timeout={AUTO_LOCK_TIMEOUT}
	/>
</footer>
