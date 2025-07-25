import Try from '@borisnl/tried';
import ky from 'ky';
import { Meel, MeelError, SentMeel, SentMeelConstructor } from '.';

/**
 * MeelSender is a class that sends Meel instances to a specified base url.
 *
 * @example
 * import { Meel, MeelSender } from "meel";
 *
 * const sender = new MeelSender({ baseUrl: "http://localhost:8080" });
 *
 * const mail = new Meel({
 * 	recipient: "Boris <boris@example.com>",
 * 	sender: "me@example.com",
 * 	template: "hello",
 * 	data: {
 * 		name: "Boris",
 * 		things: ["a", "b", "c"]
 * 	},
 * });
 *
 * const sentMail = await sender.send(mail);
 */
export class MeelSender {
	private readonly baseUrl: string;

	public constructor({ baseUrl }: { baseUrl: string }) {
		this.baseUrl = baseUrl.endsWith('/') ? baseUrl.slice(0, -1) : baseUrl;
	}

	/**
	 * Send a single Meel instance to the API.
	 *
	 * @param {Meel} mail Meel instance to send
	 * @return {Promise<SentMeel>} SentMeel instance
	 * @throws {MeelError} If the mail could not be sent
	 */
	public async send(mail: Meel): Promise<SentMeel> {
		return this.batchSend([mail]).then(data => data[0]);
	}

	/**
	 * Batch send multiple Meel instances to the API.
	 *
	 * @param {Meel[]} mails Meel instances to send
	 * @returns {Promise<SentMeel>} SentMeel instance
	 * @throws {MeelError} If the mail could not be sent
	 */
	public async batchSend(mails: Meel[]): Promise<SentMeel[]> {
		const response = await Try(() =>
			ky
				.post<SentMeelConstructor[]>(`${this.baseUrl}/mails/send`, {
					body: JSON.stringify(
						mails.map(mail => mail.toPlainObject()),
					),
					headers: {
						'Content-Type': 'application/json',
					},
				})
				.json(),
		);

		if (!response) {
			throw new MeelError('Failed to send mail', 500);
		}

		return response.map((data: SentMeelConstructor) => new SentMeel(data));
	}
}
