<script lang="ts">
	import { Field, Form, Textarea, createForm } from "svelte-forms-lib";
	import * as yup from "yup";

	import type { Task } from "$lib/model";
	import { ISO_8601_DATE_REGEX, getToday } from "$lib/model";

	export let task: Task | null = null;
	export let isOpen: boolean;
	export let saveTask: (task: Task) => void | Promise<void>;
	export let onDone: () => void;

	type FormValues = Pick<Task, "description" | "details" | "deadline">;

	const initialValues: FormValues = {
		description: task?.description ?? "",
		details: task?.details ?? "",
		deadline: task?.deadline ?? getToday(),
	};

	const validationSchema: yup.ObjectSchema<FormValues> = yup.object({
		description: yup.string().trim().required("Task description is required."),
		details: yup.string().notRequired().ensure(),
		deadline: yup.string().required().matches(ISO_8601_DATE_REGEX),
	});

	const onSubmit = async ({
		description,
		details,
		deadline,
	}: FormValues): Promise<void> => {
		const trimmedDescription = description.trim();
		if (trimmedDescription.length === 0) {
			throw new Error("Invalid task info");
		}
		const taskToSave: Task = {
			id: task?.id ?? Date.now(),
			description: trimmedDescription,
			details: details.trim(),
			deadline: deadline,
			completed: task?.completed ?? false,
		};
		saveTask(taskToSave);
		context.handleReset();
		onDone();
	};

	var context = createForm({
		initialValues,
		validationSchema,
		onSubmit,
	});

	$: errors = context.errors;
	$: touched = context.touched;
	$: descriptionInvalid = $touched.description ? !!$errors.description : null;
</script>

<dialog open={isOpen}>
	<Form {context}>
		<fieldset>
			<Field
				id="description"
				name="description"
				type="text"
				label="What needs to be done?"
				placeholder="What needs to be done?"
				aria-invalid={descriptionInvalid}
			/>
			{#if $errors.description}
				<small>{$errors.description}</small>
			{/if}
			<Field id="deadline" name="deadline" type="date" label="Deadline" />
			<details>
				<summary>Details</summary>
				<Textarea
					id="details"
					name="details"
					rows="5"
					label="Details"
					placeholder="Details"
				/>
			</details>
		</fieldset>
		<div class="grid">
			<button type="submit">Save</button>
			<button type="button" class="secondary" on:click={onDone}>
				Cancel
			</button>
		</div>
	</Form>
</dialog>
