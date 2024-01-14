export enum DateGroup {
	Past = "Past",
	Today = "Today",
	Tomorrow = "Tomorrow",
}

export const toIsoDateString = (date: Date): string =>
	date.toISOString().split("T")[0];

export const getToday = (): string => toIsoDateString(new Date());

export const getTomorrow = (): string => {
	const today = new Date(getToday());
	const tomorrow = new Date(today);
	tomorrow.setDate(today.getDate() + 1);
	return toIsoDateString(tomorrow);
};

export const getDayAfterTomorrow = (): string => {
	const tomorrow = new Date(getTomorrow());
	const dayAfterTomorrow = new Date(tomorrow);
	dayAfterTomorrow.setDate(tomorrow.getDate() + 1);
	return toIsoDateString(dayAfterTomorrow);
};
