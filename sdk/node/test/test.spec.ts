import { test, expect } from 'bun:test';

import { Meel, MeelSender, SentMeel } from '../src';

const baseUrl = 'http://localhost:8080';

test('send email', async () => {
	const sender = new MeelSender({ baseUrl });

	const meel = new Meel({
		subject: 'Hello world',
		recipient: 'Boris <boris@example.com>',
		sender: 'me@example.com',
		template: 'test',
		data: {
			names: ['john doe', 'jane doe'],
			redacted: false,
		},
	});

	expect(await sender.send(meel)).toBeInstanceOf(SentMeel);
});
