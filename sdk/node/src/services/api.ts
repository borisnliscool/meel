import ky from "ky";
import { Meel, SentMeel, SentMeelConstructor } from "../classes";

interface MeelSenderConstructor {
	baseUrl: string;
}

export class MeelSender {
	private _baseUrl: string;

	public get baseUrl(): string {
		return this._baseUrl;
	}

	public constructor({ baseUrl }: MeelSenderConstructor) {
		this._baseUrl = baseUrl.endsWith("/") ? baseUrl.slice(0, -1) : baseUrl;
	}

	public async send(mail: Meel): Promise<SentMeel> {
		return this.batchSend([mail]).then((data) => data[0]);
	}

	public async batchSend(mails: Meel[]): Promise<SentMeel[]> {
		const response = await ky
			.post<SentMeelConstructor[]>(`${this.baseUrl}/mails/send`, {
				body: JSON.stringify(mails.map((mail) => mail.toPlainObject())),
				headers: {
					"Content-Type": "application/json",
				},
			})
			.json();

		return response.map((data: SentMeelConstructor) => new SentMeel(data));
	}
}
