<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";

	import AutoLock from "$lib/component/AutoLock.svelte";
	import Header from "$lib/component/Header.svelte";
	import UnlockPage from "$lib/page/UnlockPage.svelte";
	import ChangePasswordPage from "$lib/page/ChangePasswordPage.svelte";
	import ChangeSettingsPage from "$lib/page/ChangeSettingsPage.svelte";
	import TasksPage from "$lib/page/TasksPage.svelte";
	import type { Config, FormattedTaskEvent, Task, TaskEvent } from "$lib/model";
	import {
		Page,
		TaskEventType,
		applyEvent,
		applyEvents,
		formatEvent,
		unformatEvent,
	} from "$lib/model";

	let alreadyExists = false;
	let config: Config | null;
	let tasks: Task[] = [];
	let page = Page.Loading;

	const unlock = async (password: string) => {
		config = await invoke("unlock", { password });
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
		config = null;
		tasks = [];
		page = Page.Unlock;
	};

	const addTask = async (task: Task) => {
		const event: TaskEvent = {
			type: TaskEventType.CreateTask,
			id: Date.now(),
			task,
		};
		tasks = applyEvent(tasks, event);
		await invoke("save_event", { event: formatEvent(event) });
	};

	const editTask = async (task: Task) => {
		const event: TaskEvent = {
			type: TaskEventType.UpdateTask,
			id: Date.now(),
			task,
		};
		tasks = applyEvent(tasks, event);
		await invoke("save_event", { event: formatEvent(event) });
	};

	const completeTask = async (taskId: number) => {
		const task = tasks.find((t) => t.id === taskId) ?? null;
		if (task === null) {
			throw new Error("Invalid task ID");
		}
		const event: TaskEvent = {
			type: TaskEventType.UpdateTask,
			id: Date.now(),
			task: {
				...task,
				completed: true,
			},
		};
		tasks = applyEvent(tasks, event);
		await invoke("save_event", { event: formatEvent(event) });
	};

	const uncompleteTask = async (taskId: number) => {
		const task = tasks.find((t) => t.id === taskId) ?? null;
		if (task === null) {
			throw new Error("Invalid task ID");
		}
		const event: TaskEvent = {
			type: TaskEventType.UpdateTask,
			id: Date.now(),
			task: {
				...task,
				completed: false,
			},
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

	const updateSettings = async (newConfig: Config) => {
		config = newConfig;
		await invoke("update_config", { newConfig });
	};

	const visit = (pageToVisit: Page) => {
		page = pageToVisit;
	};

	const visitChangePassword = visit.bind(null, Page.ChangePassword);
	const visitChangeSettings = visit.bind(null, Page.ChangeSettings);
	const visitTasks = visit.bind(null, Page.Tasks);

	$: autoLockTimeout = config?.autoLockTimeout;

	onMount(async () => {
		alreadyExists = await invoke("check_exists");
		page = Page.Unlock;
	});
</script>

<Header {page} {lock} {visitChangeSettings} {visitChangePassword} />

<main>
	{#if page === Page.Loading}
		Loading...
	{:else if page === Page.Unlock}
		<UnlockPage {alreadyExists} {createPassword} {unlock} />
	{:else if page === Page.Tasks}
		<TasksPage
			{tasks}
			{addTask}
			{editTask}
			{completeTask}
			{uncompleteTask}
			{deleteTask}
		/>
	{:else if page === Page.ChangePassword}
		<ChangePasswordPage {changePassword} {visitTasks} />
	{:else if page === Page.ChangeSettings && config !== null}
		<ChangeSettingsPage {config} {updateSettings} {visitTasks} />
	{:else}
		Not found
	{/if}
</main>

<footer>
	<AutoLock
		isUnlocked={![Page.Loading, Page.Unlock].includes(page)}
		{lock}
		timeout={autoLockTimeout}
	/>
</footer>
