import * as vscode from "vscode";
import { bracesTextDecoration, placeholderTextDecoration } from "./decorations";

export function checkForUnclosedBraces(
	document: vscode.TextDocument,
	diagnosticCollection: vscode.DiagnosticCollection
): void {
	if (document.languageId !== "meel") {
		return;
	}

	const diagnostics: vscode.Diagnostic[] = [];
	const yellowBracesRanges: vscode.Range[] = [];
	const lightBlueTextRanges: vscode.Range[] = [];
	const text = document.getText();
	const stack: number[] = [];
	const regex = /\{\{|\}\}/g;

	let match: RegExpExecArray | null;

	while ((match = regex.exec(text)) !== null) {
		const matchText = match[0];
		const matchIndex = match.index;

		switch (matchText) {
			case "{{": {
				stack.push(matchIndex);
				break;
			}
			case "}}": {
				if (stack.length > 0) {
					const openingBraceIndex = stack.pop()!;

					yellowBracesRanges.push(
						new vscode.Range(
							document.positionAt(openingBraceIndex),
							document.positionAt(openingBraceIndex + 2)
						),
						new vscode.Range(
							document.positionAt(matchIndex),
							document.positionAt(matchIndex + 2)
						)
					);

					lightBlueTextRanges.push(
						new vscode.Range(
							document.positionAt(openingBraceIndex + 2),
							document.positionAt(matchIndex)
						)
					);
				} else {
					const range = new vscode.Range(
						document.positionAt(matchIndex),
						document.positionAt(matchIndex + 2)
					);

					diagnostics.push(
						new vscode.Diagnostic(
							range,
							"Unmatched closing brace }}",
							vscode.DiagnosticSeverity.Error
						)
					);

					yellowBracesRanges.push(range);
				}
				break;
			}
		}
	}

	while (stack.length > 0) {
		const unmatchedIndex = stack.pop()!;
		const range = new vscode.Range(
			document.positionAt(unmatchedIndex),
			document.positionAt(unmatchedIndex + 2)
		);

		diagnostics.push(
			new vscode.Diagnostic(
				range,
				"Unmatched opening brace {{",
				vscode.DiagnosticSeverity.Error
			)
		);

		yellowBracesRanges.push(range);
	}

	// Update diagnostics for the document
	diagnosticCollection.set(document.uri, diagnostics);

	const editor = vscode.window.activeTextEditor;
	if (editor && editor.document.uri === document.uri) {
		editor.setDecorations(bracesTextDecoration, yellowBracesRanges);
		editor.setDecorations(placeholderTextDecoration, lightBlueTextRanges);
	}
}
