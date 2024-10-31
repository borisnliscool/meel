import ky from "ky";
import { Mail, SentMail, SentMailConstructor } from "../classes";

interface MeelAPIConstructor {
	baseUrl: string;
}

export class Meel {
	private _baseUrl: string;

	public get baseUrl(): string {
		return this._baseUrl;
	}

	public constructor({ baseUrl }: MeelAPIConstructor) {
		this._baseUrl = baseUrl;
	}

	public async send(mail: Mail): Promise<SentMail> {
		const response = await ky
			.post<SentMailConstructor>(`${this.baseUrl}/mails/send`, {
				body: JSON.stringify(mail.toPlainObject()),
			})
			.json();

		return new SentMail(response);
	}
}
