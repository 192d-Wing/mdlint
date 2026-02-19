//! Convert mkdlint fix_info to LSP code actions

use crate::types::LintError;
use std::collections::HashMap;

use super::utils::to_position;

// Import all LSP types from tower-lsp which re-exports lsp-types
use tower_lsp::lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, Diagnostic, Position, Range, TextEdit, Url,
    WorkspaceEdit,
};

/// Convert a LintError with fix_info to a CodeAction.
///
/// If `diagnostic` is provided, the action will reference it so the editor
/// can show a lightbulb specifically for that diagnostic.
pub fn fix_to_code_action(
    uri: &Url,
    error: &LintError,
    content: &str,
    diagnostic: Option<Diagnostic>,
) -> Option<CodeActionOrCommand> {
    let fix_info = error.fix_info.as_ref()?;

    let text_edit = calculate_text_edit(error, fix_info, content)?;

    let mut changes = HashMap::new();
    changes.insert(uri.clone(), vec![text_edit]);

    let workspace_edit = WorkspaceEdit {
        changes: Some(changes),
        ..Default::default()
    };

    let title = format!(
        "Fix: {} ({})",
        error.rule_description,
        error.rule_names.first().unwrap_or(&"unknown")
    );

    let code_action = CodeAction {
        title,
        kind: Some(CodeActionKind::QUICKFIX),
        edit: Some(workspace_edit),
        diagnostics: diagnostic.map(|d| vec![d]),
        ..Default::default()
    };

    Some(CodeActionOrCommand::CodeAction(code_action))
}

/// Calculate the TextEdit from FixInfo
fn calculate_text_edit(
    error: &LintError,
    fix_info: &crate::types::FixInfo,
    content: &str,
) -> Option<TextEdit> {
    let lines: Vec<&str> = content.lines().collect();

    // Determine target line
    let target_line = fix_info.line_number.unwrap_or(error.line_number);

    let line_idx = target_line.saturating_sub(1);
    let _line = lines.get(line_idx)?;

    // Handle delete entire line case
    if fix_info.delete_count == Some(-1) {
        return Some(create_delete_line_edit(target_line, lines.len()));
    }

    // Get edit column (1-based)
    let edit_col = fix_info.edit_column?;

    // Calculate start position
    let start = to_position(target_line, edit_col);

    // Calculate end position based on delete_count
    let end = if let Some(delete_count) = fix_info.delete_count {
        if delete_count > 0 {
            Position {
                line: start.line,
                character: start.character + delete_count as u32,
            }
        } else {
            start // delete_count == 0 means insert only
        }
    } else {
        start // No deletion, just insertion
    };

    let range = Range { start, end };
    let new_text = fix_info.insert_text.clone().unwrap_or_default();

    Some(TextEdit { range, new_text })
}

/// Create a TextEdit that deletes an entire line (including newline)
fn create_delete_line_edit(line_number: usize, total_lines: usize) -> TextEdit {
    let line_idx = line_number.saturating_sub(1);

    // Delete the entire line including newline
    let start = Position {
        line: line_idx as u32,
        character: 0,
    };

    // If this is not the last line, delete up to start of next line
    // If it is the last line, delete to end of line
    let end = if line_number < total_lines {
        Position {
            line: (line_idx + 1) as u32,
            character: 0,
        }
    } else {
        Position {
            line: line_idx as u32,
            character: u32::MAX, // Delete to end of line
        }
    };

    TextEdit {
        range: Range { start, end },
        new_text: String::new(),
    }
}

/// Compute the Levenshtein edit distance between two strings.
fn edit_distance(a: &str, b: &str) -> usize {
    let b_len = b.len();
    let mut prev: Vec<usize> = (0..=b_len).collect();
    let mut curr = vec![0; b_len + 1];

    for (i, ca) in a.chars().enumerate() {
        curr[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            curr[j + 1] = (prev[j + 1] + 1).min(curr[j] + 1).min(prev[j] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[b_len]
}

/// Build code actions for MD051 broken link errors.
///
/// Parses the `error_context` to locate the broken fragment, then suggests
/// the closest matching heading anchors as replacement quick fixes.
pub fn md051_code_actions(
    uri: &Url,
    error: &LintError,
    content: &str,
    available_headings: &[String],
    diagnostic: Option<Diagnostic>,
    max_suggestions: usize,
) -> Vec<CodeActionOrCommand> {
    let context = match &error.error_context {
        Some(ctx) => ctx.as_str(),
        None => return vec![],
    };

    // Extract the broken fragment from error_context
    // Same-file:  "[text](#broken-fragment)"
    // Cross-file: "[text](file.md#broken-fragment)"
    let fragment = if let Some(hash_pos) = context.rfind('#') {
        let after_hash = &context[hash_pos + 1..];
        after_hash.trim_end_matches(')')
    } else {
        return vec![];
    };

    if fragment.is_empty() || available_headings.is_empty() {
        return vec![];
    }

    // Find the fragment's position in the source line
    let lines: Vec<&str> = content.lines().collect();
    let error_line_idx = error.line_number.saturating_sub(1);
    let line = match lines.get(error_line_idx) {
        Some(l) => *l,
        None => return vec![],
    };

    let search_pattern = format!("#{}", fragment);
    let hash_col = match line.find(&search_pattern) {
        Some(pos) => pos,
        None => return vec![],
    };
    let frag_start_col = hash_col + 1; // after the '#'
    let frag_end_col = frag_start_col + fragment.len();

    // Rank available headings by edit distance
    let mut scored: Vec<(usize, &String)> = available_headings
        .iter()
        .map(|h| (edit_distance(fragment, h), h))
        .collect();
    scored.sort_by_key(|(dist, _)| *dist);

    // Build code actions for the top N suggestions
    let mut actions = Vec::new();
    for (_dist, heading) in scored.into_iter().take(max_suggestions) {
        let text_edit = TextEdit {
            range: Range {
                start: Position {
                    line: error_line_idx as u32,
                    character: frag_start_col as u32,
                },
                end: Position {
                    line: error_line_idx as u32,
                    character: frag_end_col as u32,
                },
            },
            new_text: heading.clone(),
        };

        let mut changes = HashMap::new();
        changes.insert(uri.clone(), vec![text_edit]);

        let code_action = CodeAction {
            title: format!("MD051: Replace with #{}", heading),
            kind: Some(CodeActionKind::QUICKFIX),
            edit: Some(WorkspaceEdit {
                changes: Some(changes),
                ..Default::default()
            }),
            diagnostics: diagnostic.as_ref().map(|d| vec![d.clone()]),
            ..Default::default()
        };
        actions.push(CodeActionOrCommand::CodeAction(code_action));
    }
    actions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FixInfo, Severity};

    fn create_test_error_with_fix(fix_info: FixInfo) -> LintError {
        LintError {
            line_number: 1,
            rule_names: &["MD001"],
            rule_description: "Test rule",
            error_detail: None,
            error_context: None,
            rule_information: None,
            error_range: None,
            fix_info: Some(fix_info),
            suggestion: Some("Apply fix".to_string()),
            severity: Severity::Error,
            fix_only: false,
        }
    }

    #[test]
    fn test_insert_text_fix() {
        let fix_info = FixInfo {
            line_number: None,
            edit_column: Some(3),
            delete_count: None,
            insert_text: Some(" ".to_string()),
        };

        let error = create_test_error_with_fix(fix_info);
        let content = "# Test\n";
        let uri = Url::parse("file:///tmp/test.md").unwrap();

        let action = fix_to_code_action(&uri, &error, content, None);
        assert!(action.is_some());

        if let Some(CodeActionOrCommand::CodeAction(ca)) = action {
            assert_eq!(ca.kind, Some(CodeActionKind::QUICKFIX));
            assert!(ca.title.contains("Test rule"));

            let edit = ca.edit.unwrap();
            let changes = edit.changes.unwrap();
            let text_edits = changes.get(&uri).unwrap();
            assert_eq!(text_edits.len(), 1);

            let text_edit = &text_edits[0];
            assert_eq!(text_edit.range.start, Position::new(0, 2));
            assert_eq!(text_edit.range.end, Position::new(0, 2));
            assert_eq!(text_edit.new_text, " ");
        }
    }

    #[test]
    fn test_delete_chars_fix() {
        let fix_info = FixInfo {
            line_number: None,
            edit_column: Some(3),
            delete_count: Some(2),
            insert_text: None,
        };

        let error = create_test_error_with_fix(fix_info);
        let content = "#  Test\n"; // Two spaces
        let uri = Url::parse("file:///tmp/test.md").unwrap();

        let action = fix_to_code_action(&uri, &error, content, None);
        assert!(action.is_some());

        if let Some(CodeActionOrCommand::CodeAction(ca)) = action {
            let edit = ca.edit.unwrap();
            let changes = edit.changes.unwrap();
            let text_edits = changes.get(&uri).unwrap();
            let text_edit = &text_edits[0];

            assert_eq!(text_edit.range.start, Position::new(0, 2));
            assert_eq!(text_edit.range.end, Position::new(0, 4));
            assert_eq!(text_edit.new_text, "");
        }
    }

    #[test]
    fn test_replace_text_fix() {
        let fix_info = FixInfo {
            line_number: None,
            edit_column: Some(1),
            delete_count: Some(9),
            insert_text: Some("## Heading".to_string()),
        };

        let error = create_test_error_with_fix(fix_info);
        let content = "_Heading_\n";
        let uri = Url::parse("file:///tmp/test.md").unwrap();

        let action = fix_to_code_action(&uri, &error, content, None);
        assert!(action.is_some());

        if let Some(CodeActionOrCommand::CodeAction(ca)) = action {
            let edit = ca.edit.unwrap();
            let changes = edit.changes.unwrap();
            let text_edits = changes.get(&uri).unwrap();
            let text_edit = &text_edits[0];

            assert_eq!(text_edit.range.start, Position::new(0, 0));
            assert_eq!(text_edit.range.end, Position::new(0, 9));
            assert_eq!(text_edit.new_text, "## Heading");
        }
    }

    #[test]
    fn test_delete_line_fix() {
        let fix_info = FixInfo {
            line_number: Some(2),
            edit_column: Some(1),
            delete_count: Some(-1),
            insert_text: None,
        };

        let error = create_test_error_with_fix(fix_info);
        let content = "> line 1\n\n> line 2\n";
        let uri = Url::parse("file:///tmp/test.md").unwrap();

        let action = fix_to_code_action(&uri, &error, content, None);
        assert!(action.is_some());

        if let Some(CodeActionOrCommand::CodeAction(ca)) = action {
            let edit = ca.edit.unwrap();
            let changes = edit.changes.unwrap();
            let text_edits = changes.get(&uri).unwrap();
            let text_edit = &text_edits[0];

            // Should delete line 2 (index 1) up to start of line 3
            assert_eq!(text_edit.range.start, Position::new(1, 0));
            assert_eq!(text_edit.range.end, Position::new(2, 0));
            assert_eq!(text_edit.new_text, "");
        }
    }

    #[test]
    fn test_no_fix_info() {
        let mut error = create_test_error_with_fix(FixInfo {
            line_number: None,
            edit_column: None,
            delete_count: None,
            insert_text: None,
        });
        error.fix_info = None;

        let content = "# Test\n";
        let uri = Url::parse("file:///tmp/test.md").unwrap();

        let action = fix_to_code_action(&uri, &error, content, None);
        assert!(action.is_none());
    }

    #[test]
    fn test_edit_distance() {
        assert_eq!(edit_distance("", ""), 0);
        assert_eq!(edit_distance("abc", "abc"), 0);
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        assert_eq!(edit_distance("introductoin", "introduction"), 2);
        assert_eq!(edit_distance("", "abc"), 3);
        assert_eq!(edit_distance("abc", ""), 3);
    }

    #[test]
    fn test_md051_code_actions_same_file() {
        let uri = Url::parse("file:///tmp/test.md").unwrap();
        let error = LintError {
            line_number: 3,
            rule_names: &["MD051", "link-fragments"],
            rule_description: "Link fragments should be valid",
            error_detail: Some("No matching heading for fragment: #introductoin".to_string()),
            error_context: Some("[link](#introductoin)".to_string()),
            rule_information: None,
            error_range: None,
            fix_info: None,
            suggestion: None,
            severity: Severity::Error,
            fix_only: false,
        };
        let content = "# Introduction\n\n[link](#introductoin)\n";
        let headings = vec![
            "introduction".to_string(),
            "getting-started".to_string(),
            "api-reference".to_string(),
        ];

        let actions = md051_code_actions(&uri, &error, content, &headings, None, 3);
        assert!(!actions.is_empty(), "Should produce code actions");

        // First suggestion should be the closest match: "introduction"
        if let CodeActionOrCommand::CodeAction(ca) = &actions[0] {
            assert!(
                ca.title.contains("introduction"),
                "First action should suggest 'introduction', got: {}",
                ca.title
            );
            let edit = ca.edit.as_ref().unwrap();
            let changes = edit.changes.as_ref().unwrap();
            let edits = changes.get(&uri).unwrap();
            assert_eq!(edits[0].new_text, "introduction");
        }
    }

    #[test]
    fn test_md051_code_actions_no_context() {
        let uri = Url::parse("file:///tmp/test.md").unwrap();
        let error = LintError {
            line_number: 1,
            rule_names: &["MD051"],
            rule_description: "Link fragments should be valid",
            error_detail: None,
            error_context: None,
            rule_information: None,
            error_range: None,
            fix_info: None,
            suggestion: None,
            severity: Severity::Error,
            fix_only: false,
        };
        let actions = md051_code_actions(&uri, &error, "# Test\n", &["test".to_string()], None, 3);
        assert!(actions.is_empty(), "No context should produce no actions");
    }

    #[test]
    fn test_md051_code_actions_empty_headings() {
        let uri = Url::parse("file:///tmp/test.md").unwrap();
        let error = LintError {
            line_number: 1,
            rule_names: &["MD051"],
            rule_description: "Link fragments should be valid",
            error_detail: Some("No matching heading for fragment: #broken".to_string()),
            error_context: Some("[link](#broken)".to_string()),
            rule_information: None,
            error_range: None,
            fix_info: None,
            suggestion: None,
            severity: Severity::Error,
            fix_only: false,
        };
        let actions = md051_code_actions(&uri, &error, "[link](#broken)\n", &[], None, 3);
        assert!(
            actions.is_empty(),
            "Empty headings should produce no actions"
        );
    }
}
