import { MeelPriority } from "../types";
import { removeUndefinedValues } from "../utility";

export interface MeelConstructor {
	recipient: string;
	sender: string;
	template: string;
	data: Record<string, string>;
	priority?: MeelPriority | number;
	allow_html?: boolean;
	minify_html?: boolean;
	schedule_at?: string | Date;
	reply_to?: string;
	subject?: string;
}

export class Meel {
	public recipient: string;
	public sender: string;
	public template: string;
	public priority: MeelPriority | number;
	public data: Record<string, string>;
	public allow_html?: boolean;
	public minify_html?: boolean;
	public schedule_at?: Date;
	public reply_to?: string;
	public subject?: string;

	public constructor(data: MeelConstructor) {
		this.recipient = data.recipient;
		this.sender = data.sender;
		this.subject = data.subject;
		this.template = data.template;
		this.priority = data.priority ?? MeelPriority.NORMAL;
		this.data = data.data ?? {};
		this.allow_html = data.allow_html;
		this.minify_html = data.minify_html;
		this.reply_to = data.reply_to;
		this.schedule_at = data.schedule_at
			? data.schedule_at instanceof Date
				? data.schedule_at
				: new Date(data.schedule_at)
			: undefined;
	}

	public toPlainObject(): MeelConstructor {
		return removeUndefinedValues<MeelConstructor>({
			recipient: this.recipient,
			sender: this.sender,
			template: this.template,
			priority: this.priority,
			data: this.data,
			allow_html: this.allow_html,
			minify_html: this.minify_html,
			schedule_at: this.schedule_at?.toISOString(),
			reply_to: this.reply_to,
			subject: this.subject,
		});
	}
}
