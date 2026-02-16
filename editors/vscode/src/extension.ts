import * as vscode from "vscode";
import { LanguageClient } from "vscode-languageclient/node";
import { createClient, startClient, stopClient } from "./client";
import { registerCommands } from "./commands";
import { createStatusBarItem, updateStatusBar } from "./statusBar";

let client: LanguageClient | undefined;

export async function activate(
  context: vscode.ExtensionContext
): Promise<void> {
  const config = vscode.workspace.getConfiguration("mkdlint");
  if (!config.get<boolean>("enable", true)) {
    return;
  }

  client = createClient(context);
  if (!client) {
    return;
  }

  registerCommands(context, client);

  const statusBarItem = createStatusBarItem();
  context.subscriptions.push(statusBarItem);

  context.subscriptions.push(
    vscode.languages.onDidChangeDiagnostics(() => {
      updateStatusBar(statusBarItem);
    })
  );

  context.subscriptions.push(
    vscode.window.onDidChangeActiveTextEditor(() => {
      updateStatusBar(statusBarItem);
    })
  );

  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration((e) => {
      if (e.affectsConfiguration("mkdlint.enable")) {
        const nowEnabled = vscode.workspace
          .getConfiguration("mkdlint")
          .get<boolean>("enable", true);
        if (!nowEnabled && client) {
          stopClient(client);
          client = undefined;
        }
      }
    })
  );

  await startClient(client);
}

export async function deactivate(): Promise<void> {
  if (client) {
    await stopClient(client);
  }
}
