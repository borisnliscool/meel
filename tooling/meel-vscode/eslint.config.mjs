import pluginJs from '@eslint/js';
import globals from 'globals';
import tseslint from 'typescript-eslint';

/** @type {import('eslint').Linter.Config[]} */
export default [
	{ files: ['src/**/*.{js,mjs,cjs,ts}'] },
	{ languageOptions: { globals: globals.browser } },
	pluginJs.configs.recommended,
	...tseslint.configs.recommended,
	{ ignores: ['dist/', 'esbuild.js'] },
	{
		rules: {
			"@typescript-eslint/no-unused-expressions": "off"
		}
	}
];
