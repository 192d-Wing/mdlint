import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";
import { resolveBinaryPath } from "./platform";

export function createClient(
  context: vscode.ExtensionContext
): LanguageClient | undefined {
  const binaryPath = resolveBinaryPath(context);
  if (!binaryPath) {
    vscode.window.showErrorMessage(
      "mkdlint: Could not find mkdlint-lsp binary. " +
        "Set mkdlint.path in settings or install via: cargo install mkdlint --features lsp"
    );
    return undefined;
  }

  const serverOptions: ServerOptions = {
    command: binaryPath,
    args: [],
    options: {
      env: {
        ...process.env,
        RUST_LOG: process.env.RUST_LOG || "info",
      },
    },
  };

  const config = vscode.workspace.getConfiguration("mkdlint");
  const preset = config.get<string | null>("preset") ?? null;

  const clientOptions: LanguageClientOptions = {
    documentSelector: [
      { scheme: "file", language: "markdown" },
      { scheme: "untitled", language: "markdown" },
    ],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher(
        "**/.markdownlint{.json,.jsonc,.yaml,.yml,rc}"
      ),
    },
    outputChannelName: "mkdlint",
    initializationOptions: {
      ...(preset ? { preset } : {}),
    },
  };

  return new LanguageClient(
    "mkdlint",
    "mkdlint Language Server",
    serverOptions,
    clientOptions
  );
}

export async function startClient(client: LanguageClient): Promise<void> {
  await client.start();
}

export async function stopClient(client: LanguageClient): Promise<void> {
  if (client.isRunning()) {
    await client.stop();
  }
}
