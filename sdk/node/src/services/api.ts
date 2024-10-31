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
		this._baseUrl = baseUrl.endsWith("/") ? baseUrl.slice(0, -1) : baseUrl;
	}

	public async send(mail: Mail): Promise<SentMail> {
		return this.batchSend([mail]).then((data) => data[0]);
	}

	public async batchSend(mails: Mail[]): Promise<SentMail[]> {
		const response = await ky
			.post<SentMailConstructor[]>(`${this.baseUrl}/mails/send`, {
				body: JSON.stringify(mails.map((mail) => mail.toPlainObject())),
				headers: {
					"Content-Type": "application/json",
				},
			})
			.json();

		return response.map((data) => new SentMail(data));
	}
}
