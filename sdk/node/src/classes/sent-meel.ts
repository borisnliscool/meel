import { MeelPriority } from '../types';

export interface SentMeelConstructor {
	id: number;
	sender: string;
	recipient: string;
	send_attempts: number;
	priority: number;
	scheduled_at: string;
	sent_at?: string;
	sent: boolean;
}

/**
 * SentMeel is a class that represents a mail that has been sent to a recipient.
 */
export class SentMeel {
	public readonly id: number;
	public readonly sender: string;
	public readonly recipient: string;
	public readonly send_attempts: number;
	public readonly priority: MeelPriority | number;
	public readonly sent_at?: Date;
	public readonly sent: boolean;

	public constructor(data: SentMeelConstructor) {
		this.id = data.id;
		this.sender = data.sender;
		this.recipient = data.recipient;
		this.send_attempts = data.send_attempts;
		this.priority = data.priority;
		this.sent_at = data.sent_at ? new Date(data.sent_at) : undefined;
		this.sent = data.sent;
	}
}
