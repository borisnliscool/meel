import * as vscode from "vscode";
import { createDecorations, disposeDecorations } from "./decorations";
import { checkForUnclosedBraces } from "./diagnostics";

export function activate(context: vscode.ExtensionContext) {
	const { bracesTextDecoration, placeholderTextDecoration } =
		createDecorations();

	const diagnosticCollection =
		vscode.languages.createDiagnosticCollection("unclosedBraces");

	if (vscode.window.activeTextEditor) {
		checkForUnclosedBraces(
			vscode.window.activeTextEditor.document,
			diagnosticCollection
		);
	}

	context.subscriptions.push(
		vscode.workspace.onDidOpenTextDocument((doc) => {
			checkForUnclosedBraces(doc, diagnosticCollection);

			if (vscode.window.activeTextEditor?.document === doc) {
				checkForUnclosedBraces(doc, diagnosticCollection);
			}
		}),
		vscode.workspace.onDidChangeTextDocument((event) =>
			checkForUnclosedBraces(event.document, diagnosticCollection)
		),
		vscode.window.onDidChangeActiveTextEditor((editor) => {
			if (editor) {
				checkForUnclosedBraces(editor.document, diagnosticCollection);
			}
		}),
		diagnosticCollection,
		bracesTextDecoration,
		placeholderTextDecoration
	);
}

export function deactivate() {
	disposeDecorations();
}
