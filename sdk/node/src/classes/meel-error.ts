export class MeelError extends Error {
	public readonly message: string;
	public readonly code: number;

	public constructor(message: string, code: number) {
		super(message);
		this.message = message;
		this.code = code;
	}

	public toString(): string {
		return `${this.code}: ${this.message}`;
	}
}
