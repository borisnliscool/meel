export interface MeelErrorConstructor {
	status_code: number;
	error_code: number;
	message: string;
	details: object;
}

export class MeelError extends Error {
	public readonly message: string;
	public readonly code: number;
	public readonly details: object;
	public readonly error_code: number;

	public constructor(data: MeelErrorConstructor) {
		super(data.message);
		this.message = data.message;
		this.code = data.status_code;
		this.details = data.details;
		this.error_code = data.error_code;
	}

	public toString(): string {
		return `${this.code}: ${this.message}`;
	}
}
