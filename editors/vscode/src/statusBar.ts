import * as vscode from "vscode";

export function createStatusBarItem(): vscode.StatusBarItem {
  const item = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Right,
    100
  );
  item.command = "mkdlint.showOutput";
  item.name = "mkdlint";
  updateStatusBar(item);
  item.show();
  return item;
}

export function updateStatusBar(item: vscode.StatusBarItem): void {
  const editor = vscode.window.activeTextEditor;
  if (!editor || editor.document.languageId !== "markdown") {
    item.hide();
    return;
  }

  const diagnostics = vscode.languages.getDiagnostics(editor.document.uri);
  const mkdlintDiags = diagnostics.filter((d) => d.source === "mkdlint");

  const errors = mkdlintDiags.filter(
    (d) => d.severity === vscode.DiagnosticSeverity.Error
  ).length;
  const warnings = mkdlintDiags.filter(
    (d) => d.severity === vscode.DiagnosticSeverity.Warning
  ).length;

  if (errors === 0 && warnings === 0) {
    item.text = "$(check) mkdlint";
    item.tooltip = "mkdlint: No issues";
    item.backgroundColor = undefined;
  } else {
    const parts: string[] = [];
    if (errors > 0) parts.push(`$(error) ${errors}`);
    if (warnings > 0) parts.push(`$(warning) ${warnings}`);
    item.text = `mkdlint: ${parts.join(" ")}`;
    item.tooltip = `mkdlint: ${errors} error(s), ${warnings} warning(s)`;
    if (errors > 0) {
      item.backgroundColor = new vscode.ThemeColor(
        "statusBarItem.errorBackground"
      );
    } else {
      item.backgroundColor = new vscode.ThemeColor(
        "statusBarItem.warningBackground"
      );
    }
  }

  item.show();
}
