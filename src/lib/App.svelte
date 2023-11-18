<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";

	import AutoLock from "$lib/component/AutoLock.svelte";
	import ChangePasswordForm from "$lib/component/ChangePasswordForm.svelte";
	import Header from "$lib/component/Header.svelte";
	import UnlockForm from "$lib/component/UnlockForm.svelte";
	import Tasks from "$lib/page/Tasks.svelte";
	import { AUTO_LOCK_TIMEOUT } from "$lib/constant";
	import {
		TaskEventType,
		type TaskEvent,
		applyEvent,
		applyEvents,
		formatEvent,
		unformatEvent,
		type FormattedTaskEvent,
	} from "$lib/event";
	import { Page } from "$lib/page";
	import type { Task } from "$lib/task";

	let alreadyExists = false;
	let page = Page.Loading;
	let tasks: Task[] = [];

	const unlock = async (password: string) => {
		await invoke("unlock", { password });
		try {
			const formattedEvents: readonly FormattedTaskEvent[] = await invoke(
				"load_events"
			);
			const events = formattedEvents.map(unformatEvent);
			tasks = applyEvents([], events);
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
		const id = Date.now();
		const task = createTask(id, description, deadline, details);
		const event: TaskEvent = {
			type: TaskEventType.CreateTask,
			id,
			task,
		};
		tasks = applyEvent(tasks, event);
		await invoke("save_event", { event: formatEvent(event) });
	};

	const editTask = async (
		taskId: number,
		description: string,
		deadline: string,
		details: string
	) => {
		const edit = createTask(taskId, description, deadline, details);
		const event: TaskEvent = {
			type: TaskEventType.EditTask,
			id: Date.now(),
			taskId,
			edit,
		};
		tasks = applyEvent(tasks, event);
		await invoke("save_event", { event: formatEvent(event) });
	};

	const completeTask = async (taskId: number) => {
		const event: TaskEvent = {
			type: TaskEventType.CompleteTask,
			id: Date.now(),
			taskId,
		};
		tasks = applyEvent(tasks, event);
		await invoke("save_event", { event: formatEvent(event) });
	};

	const uncompleteTask = async (taskId: number) => {
		const event: TaskEvent = {
			type: TaskEventType.UncompleteTask,
			id: Date.now(),
			taskId,
		};
		tasks = applyEvent(tasks, event);
		await invoke("save_event", { event: formatEvent(event) });
	};

	const deleteTask = async (taskId: number) => {
		const event: TaskEvent = {
			type: TaskEventType.DeleteTask,
			id: Date.now(),
			taskId,
		};
		tasks = applyEvent(tasks, event);
		await invoke("save_event", { event: formatEvent(event) });
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
		<Tasks
			{tasks}
			{addTask}
			{editTask}
			{completeTask}
			{uncompleteTask}
			{deleteTask}
		/>
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
