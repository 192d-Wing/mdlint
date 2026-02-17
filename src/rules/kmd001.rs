//! KMD001 - Definition list terms must have definitions
//!
//! In Kramdown, a definition list looks like:
//!
//! ```text
//! term
//! : definition
//! ```
//!
//! This rule fires when a line that looks like a DL term (non-empty, not a
//! block-level marker) is followed by a blank line or EOF without any
//! `: definition` line.

use crate::types::{LintError, ParserType, Rule, RuleParams, Severity};

pub struct KMD001;

/// Heuristic: a line is a potential DL term if it is non-empty, not indented,
/// and does not start with a block-level character.
fn looks_like_dl_term(line: &str) -> bool {
    let trimmed = line.trim_end_matches('\n').trim_end_matches('\r');
    if trimmed.is_empty() {
        return false;
    }
    // Must not be indented
    if line.starts_with(' ') || line.starts_with('\t') {
        return false;
    }
    // Must not start with a block-level marker
    let first = trimmed.chars().next().unwrap_or(' ');
    !matches!(
        first,
        ':' | '#' | '-' | '*' | '+' | '>' | '`' | '~' | '|' | '!'
    ) && !trimmed.starts_with("```")
        && !trimmed.starts_with("~~~")
        && !trimmed.starts_with("<!--")
        && !trimmed.starts_with('[')
        && !trimmed.starts_with("---")
        && !trimmed.starts_with("===")
        && !trimmed.starts_with("***")
}

/// Returns true if the line is a Kramdown definition line (starts with `: `).
fn is_definition_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with(": ") || trimmed == ":"
}

impl Rule for KMD001 {
    fn names(&self) -> &'static [&'static str] {
        &["KMD001", "definition-list-term-has-definition"]
    }

    fn description(&self) -> &'static str {
        "Definition list terms must be followed by a definition"
    }

    fn tags(&self) -> &[&'static str] {
        &["kramdown", "definition-lists"]
    }

    fn parser_type(&self) -> ParserType {
        ParserType::None
    }

    fn is_enabled_by_default(&self) -> bool {
        false
    }

    fn lint(&self, params: &RuleParams) -> Vec<LintError> {
        let mut errors = Vec::new();
        let lines = params.lines;
        let mut in_code_block = false;

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];
            let trimmed = line.trim_end_matches('\n').trim_end_matches('\r');

            // Track code fences
            if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                in_code_block = !in_code_block;
                i += 1;
                continue;
            }
            if in_code_block {
                i += 1;
                continue;
            }

            if looks_like_dl_term(line) {
                // Look ahead for a definition line, skipping only blank lines
                // that might separate term from definition (not standard Kramdown,
                // but be lenient — require at least one `: def` within 3 lines).
                let mut found_def = false;
                let mut j = i + 1;
                while j < lines.len() && j <= i + 3 {
                    let next = lines[j].trim_end_matches('\n').trim_end_matches('\r');
                    if is_definition_line(lines[j]) {
                        found_def = true;
                        break;
                    }
                    if next.is_empty() {
                        j += 1;
                        continue;
                    }
                    // Non-empty, non-definition line → term has no definition
                    break;
                }

                if !found_def {
                    // Only report if the NEXT non-empty line is a `: ` line
                    // somewhere — i.e., at least one DL exists in this doc —
                    // to avoid false positives on plain paragraphs.
                    // Look for any `: ` line in the whole document.
                    let doc_has_any_dl = lines.iter().any(|l| is_definition_line(l));
                    if doc_has_any_dl {
                        errors.push(LintError {
                            line_number: i + 1,
                            rule_names: self.names(),
                            rule_description: self.description(),
                            error_detail: Some("Term has no definition".to_string()),
                            severity: Severity::Error,
                            ..Default::default()
                        });
                    }
                }
            }

            i += 1;
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::RuleParams;
    use std::collections::HashMap;

    fn lint(content: &str) -> Vec<LintError> {
        let lines: Vec<&str> = content.split_inclusive('\n').collect();
        let rule = KMD001;
        rule.lint(&RuleParams {
            name: "test.md",
            version: "0",
            lines: &lines,
            front_matter_lines: &[],
            tokens: &[],
            config: &HashMap::new(),
        })
    }

    #[test]
    fn test_kmd001_term_with_definition_ok() {
        let errors = lint("# H\n\nterm\n: definition\n");
        assert!(
            errors.is_empty(),
            "should not fire when term has definition"
        );
    }

    #[test]
    fn test_kmd001_term_no_definition() {
        let errors = lint("# H\n\nterm without def\n\nother paragraph\n: orphan def\n");
        assert!(
            errors.iter().any(|e| e.rule_names[0] == "KMD001"),
            "should fire when DL term has no definition"
        );
    }

    #[test]
    fn test_kmd001_no_dl_no_error() {
        // No `: ` lines at all → should not fire (no DL in document)
        let errors = lint("# H\n\nPlain paragraph.\n");
        assert!(errors.is_empty(), "should not fire when no DL in document");
    }

    #[test]
    fn test_kmd001_in_code_block_ignored() {
        let errors = lint("# H\n\n```\nterm\n: def inside code\n```\n");
        assert!(errors.is_empty(), "should not fire inside code blocks");
    }
}
