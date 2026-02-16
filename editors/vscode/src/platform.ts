import * as vscode from "vscode";
import * as path from "path";
import * as fs from "fs";
import * as os from "os";

function getBinaryName(): string {
  return os.platform() === "win32" ? "mkdlint-lsp.exe" : "mkdlint-lsp";
}

function isSupported(): boolean {
  const key = `${os.platform()}-${os.arch()}`;
  return [
    "linux-x64",
    "linux-arm64",
    "darwin-x64",
    "darwin-arm64",
    "win32-x64",
  ].includes(key);
}

/**
 * Resolve the mkdlint-lsp binary path.
 *
 * Strategy:
 * 1. User-configured path (mkdlint.path setting)
 * 2. Bundled binary in extension's server/ directory
 * 3. Binary on PATH
 */
export function resolveBinaryPath(
  context: vscode.ExtensionContext
): string | undefined {
  // 1. User-configured path
  const configPath = vscode.workspace
    .getConfiguration("mkdlint")
    .get<string | null>("path", null);
  if (configPath) {
    if (fs.existsSync(configPath)) {
      return configPath;
    }
    vscode.window.showWarningMessage(
      `mkdlint: Configured path does not exist: ${configPath}. Falling back.`
    );
  }

  // 2. Bundled binary
  if (isSupported()) {
    const bundledPath = path.join(
      context.extensionPath,
      "server",
      getBinaryName()
    );
    if (fs.existsSync(bundledPath)) {
      if (os.platform() !== "win32") {
        try {
          fs.chmodSync(bundledPath, 0o755);
        } catch {
          // Best effort
        }
      }
      return bundledPath;
    }
  }

  // 3. PATH lookup
  return which(getBinaryName());
}

function which(name: string): string | undefined {
  const pathEnv = process.env.PATH || "";
  const dirs = pathEnv.split(path.delimiter);

  for (const dir of dirs) {
    const candidate = path.join(dir, name);
    try {
      fs.accessSync(candidate, fs.constants.X_OK);
      return candidate;
    } catch {
      continue;
    }
  }
  return undefined;
}
