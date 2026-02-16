import * as vscode from "vscode";
import { LanguageClient } from "vscode-languageclient/node";

export function registerCommands(
  context: vscode.ExtensionContext,
  client: LanguageClient
): void {
  context.subscriptions.push(
    vscode.commands.registerCommand("mkdlint.fixAll", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || editor.document.languageId !== "markdown") {
        vscode.window.showWarningMessage("mkdlint: No active markdown file.");
        return;
      }
      if (!client.isRunning()) {
        vscode.window.showWarningMessage(
          "mkdlint: Language server is not running."
        );
        return;
      }
      await client.sendRequest("workspace/executeCommand", {
        command: "mkdlint.fixAll",
        arguments: [editor.document.uri.toString()],
      });
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("mkdlint.restart", async () => {
      if (client.isRunning()) {
        await client.restart();
        vscode.window.showInformationMessage(
          "mkdlint: Language server restarted."
        );
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("mkdlint.showOutput", () => {
      client.outputChannel.show();
    })
  );
}
