//! Plain text output formatter

use crate::types::{LintResults, Severity};
use colored::Colorize;

/// Format lint results as colored text with summary
pub fn format_text(results: &LintResults) -> String {
    let mut output = Vec::new();
    let mut files: Vec<_> = results.results.keys().collect();
    files.sort();

    for file in &files {
        if let Some(errors) = results.results.get(*file) {
            for error in errors {
                let rule_moniker = error.rule_names.join("/");

                let colored_rule = match error.severity {
                    Severity::Error => rule_moniker.red().to_string(),
                    Severity::Warning => rule_moniker.yellow().to_string(),
                };

                let mut line = format!(
                    "{}: {}: {} {}",
                    file.cyan(),
                    error.line_number.to_string().yellow(),
                    colored_rule,
                    error.rule_description
                );

                if let Some(detail) = &error.error_detail {
                    line.push_str(&format!(" {}", format!("[{}]", detail).dimmed()));
                }

                if let Some(context) = &error.error_context {
                    line.push_str(&format!(" {}", format!("[Context: \"{}\"]", context).dimmed()));
                }

                output.push(line);
            }
        }
    }

    // Summary line
    let error_count = results.error_count();
    let warning_count = results.warning_count();
    let file_count = results.files_with_errors().len();

    if error_count > 0 || warning_count > 0 {
        output.push(String::new());
        let summary = format!(
            "{} error(s), {} warning(s) in {} file(s)",
            error_count, warning_count, file_count
        );
        output.push(summary.bold().to_string());
    }

    output.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::LintError;

    #[test]
    fn test_format_text_empty() {
        let results = LintResults::new();
        assert_eq!(format_text(&results), "");
    }

    #[test]
    fn test_format_text_with_errors() {
        colored::control::set_override(false);
        let mut results = LintResults::new();
        results.add(
            "test.md".to_string(),
            vec![LintError {
                line_number: 1,
                rule_names: vec!["MD001".to_string(), "heading-increment".to_string()],
                rule_description: "Heading levels should increment by one".to_string(),
                severity: Severity::Error,
                ..Default::default()
            }],
        );
        let output = format_text(&results);
        assert!(output.contains("test.md"));
        assert!(output.contains("MD001"));
    }

    #[test]
    fn test_format_text_summary() {
        colored::control::set_override(false);
        let mut results = LintResults::new();
        results.add(
            "test.md".to_string(),
            vec![
                LintError {
                    line_number: 1,
                    rule_names: vec!["MD001".to_string()],
                    rule_description: "test".to_string(),
                    severity: Severity::Error,
                    ..Default::default()
                },
                LintError {
                    line_number: 2,
                    rule_names: vec!["MD059".to_string()],
                    rule_description: "test".to_string(),
                    severity: Severity::Warning,
                    ..Default::default()
                },
            ],
        );
        let output = format_text(&results);
        assert!(output.contains("1 error(s), 1 warning(s) in 1 file(s)"));
    }
}
