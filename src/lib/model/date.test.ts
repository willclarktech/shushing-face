import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
	getToday,
	getTomorrow,
	getDayAfterTomorrow,
	toIsoDateString,
} from "./date";

describe("date", () => {
	beforeEach(() => {
		const mockDate = new Date("2023-01-01T12:00:00Z");
		vi.useFakeTimers();
		vi.setSystemTime(mockDate);
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	describe("toIsoDateString", () => {
		it("converts a Date to an ISO string", () => {
			const date = new Date("2023-01-01T12:00:00Z");
			const isoDate = toIsoDateString(date);
			expect(isoDate).toBe("2023-01-01");
		});
	});

	describe("getToday", () => {
		it("returns the current date in ISO format", () => {
			const today = getToday();
			expect(today).toBe("2023-01-01");
		});
	});

	describe("getTomorrow", () => {
		it("returns the next date in ISO format", () => {
			const tomorrow = getTomorrow();
			expect(tomorrow).toBe("2023-01-02");
		});
	});

	describe("getDayAfterTomorrow", () => {
		it("returns the day after the next date in ISO format", () => {
			const dayAfterTomorrow = getDayAfterTomorrow();
			expect(dayAfterTomorrow).toBe("2023-01-03");
		});
	});
});
