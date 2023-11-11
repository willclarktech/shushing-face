export interface Task {
	id: number;
	description: string;
	details: string;
	deadline: number;
	completed: boolean;
}

export enum Page {
	Loading,
	Unlock,
	Tasks,
	ChangePassword,
}
