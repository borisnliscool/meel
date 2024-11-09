import * as vscode from 'vscode';

export const bracesTextDecoration =
	vscode.window.createTextEditorDecorationType({
		color: '#FFD700',
	});

export const placeholderTextDecoration =
	vscode.window.createTextEditorDecorationType({
		color: '#9CDCFE',
	});

export function createDecorations() {
	return {
		bracesTextDecoration,
		placeholderTextDecoration,
	};
}

export function disposeDecorations() {
	bracesTextDecoration.dispose();
	placeholderTextDecoration.dispose();
}
