//! MD046 - Code block style
//!
//! Supports `style` config: "consistent" (default), "fenced", or "indented".
//! - "consistent": all code blocks must use the same style as the first one found
//! - "fenced": all code blocks must be fenced (``` or ~~~)
//! - "indented": all code blocks must be indented (4 spaces)

use crate::types::{LintError, ParserType, Rule, RuleParams, Severity};
use once_cell::sync::Lazy;
use regex::Regex;

static CODE_FENCE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\s*)(`{3,}|~{3,})").unwrap());

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockStyle {
    Fenced,
    Indented,
}

/// A detected code block with its style and starting line.
struct CodeBlock {
    style: BlockStyle,
    start_line: usize,
}

pub struct MD046;

impl Rule for MD046 {
    fn names(&self) -> &'static [&'static str] {
        &["MD046", "code-block-style"]
    }

    fn description(&self) -> &'static str {
        "Code block style"
    }

    fn tags(&self) -> &[&'static str] {
        &["code"]
    }

    fn parser_type(&self) -> ParserType {
        ParserType::None
    }

    fn information(&self) -> Option<&'static str> {
        Some("https://github.com/DavidAnson/markdownlint/blob/main/doc/md046.md")
    }

    fn lint(&self, params: &RuleParams) -> Vec<LintError> {
        let style_str = params
            .config
            .get("style")
            .and_then(|v| v.as_str())
            .unwrap_or("consistent");

        let required_style = match style_str {
            "fenced" => Some(BlockStyle::Fenced),
            "indented" => Some(BlockStyle::Indented),
            _ => None, // "consistent" — determined by first block
        };

        // Collect all code blocks
        let blocks = find_code_blocks(params.lines);

        if blocks.is_empty() {
            return Vec::new();
        }

        // Determine the expected style
        let expected = required_style.unwrap_or(blocks[0].style);

        let expected_label = match expected {
            BlockStyle::Fenced => "fenced",
            BlockStyle::Indented => "indented",
        };

        let mut errors = Vec::new();
        for block in &blocks {
            if block.style != expected {
                let actual_label = match block.style {
                    BlockStyle::Fenced => "fenced",
                    BlockStyle::Indented => "indented",
                };
                errors.push(LintError {
                    line_number: block.start_line,
                    rule_names: self.names(),
                    rule_description: self.description(),
                    error_detail: Some(format!(
                        "Expected: {}; Actual: {}",
                        expected_label, actual_label
                    )),
                    error_context: None,
                    rule_information: self.information(),
                    error_range: None,
                    fix_info: None,
                    suggestion: Some(format!("Use {} code block style", expected_label)),
                    severity: Severity::Error,
                });
            }
        }

        errors
    }
}

/// Find all code blocks in the document, returning their style and start line.
fn find_code_blocks(lines: &[&str]) -> Vec<CodeBlock> {
    let mut blocks = Vec::new();
    let mut in_fenced = false;
    let mut fence_indent = 0;
    let mut fence_char = ' ';
    let mut fence_len = 0;
    let mut in_indented = false;
    let mut indented_start = 0;

    for (idx, line) in lines.iter().enumerate() {
        let line_number = idx + 1;
        let trimmed = line.trim_end_matches('\n').trim_end_matches('\r');

        // Check for fenced code block delimiter
        if let Some(caps) = CODE_FENCE_RE.captures(trimmed) {
            let indent = caps.get(1).unwrap().as_str().len();
            let fence = caps.get(2).unwrap().as_str();
            let fc = fence.chars().next().unwrap();
            let fl = fence.len();

            if in_fenced {
                // Closing fence: must match char, >= length, and <= indent
                if fc == fence_char && fl >= fence_len && indent <= fence_indent {
                    in_fenced = false;
                }
            } else {
                // Opening fence (only if indent < 4, per CommonMark)
                if indent < 4 {
                    // End any indented block first
                    if in_indented {
                        blocks.push(CodeBlock {
                            style: BlockStyle::Indented,
                            start_line: indented_start,
                        });
                        in_indented = false;
                    }
                    in_fenced = true;
                    fence_indent = indent;
                    fence_char = fc;
                    fence_len = fl;
                    blocks.push(CodeBlock {
                        style: BlockStyle::Fenced,
                        start_line: line_number,
                    });
                }
            }
            continue;
        }

        if in_fenced {
            continue;
        }

        // Check for indented code block (4+ spaces, not inside a list)
        // An indented code block requires a blank line before it (or start of doc)
        let is_indented_line = !trimmed.is_empty() && line.starts_with("    ");

        if is_indented_line {
            if !in_indented {
                // Check for blank line before (or start of document)
                let prev_blank = if idx == 0 {
                    true
                } else {
                    let prev = lines[idx - 1].trim_end_matches('\n').trim_end_matches('\r');
                    prev.trim().is_empty()
                };
                if prev_blank {
                    in_indented = true;
                    indented_start = line_number;
                }
            }
        } else {
            // Non-indented, non-empty line ends an indented block
            if in_indented && !trimmed.is_empty() {
                blocks.push(CodeBlock {
                    style: BlockStyle::Indented,
                    start_line: indented_start,
                });
                in_indented = false;
            }
            // Blank lines don't end the indented block (they can appear within)
        }
    }

    // Close trailing indented block
    if in_indented {
        blocks.push(CodeBlock {
            style: BlockStyle::Indented,
            start_line: indented_start,
        });
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_params<'a>(
        lines: &'a [&'a str],
        config: &'a HashMap<String, serde_json::Value>,
    ) -> RuleParams<'a> {
        RuleParams {
            name: "test.md",
            version: "0.1.0",
            lines,
            front_matter_lines: &[],
            tokens: &[],
            config,
        }
    }

    #[test]
    fn test_md046_fenced_only() {
        let lines = vec!["# Title\n", "\n", "```\n", "code\n", "```\n"];
        let config = HashMap::new();
        let params = make_params(&lines, &config);
        let errors = MD046.lint(&params);
        assert_eq!(errors.len(), 0, "Fenced-only should not trigger MD046");
    }

    #[test]
    fn test_md046_indented_only() {
        let lines = vec!["# Title\n", "\n", "    code block\n", "    more code\n"];
        let config = HashMap::new();
        let params = make_params(&lines, &config);
        let errors = MD046.lint(&params);
        assert_eq!(errors.len(), 0, "Indented-only should not trigger MD046");
    }

    #[test]
    fn test_md046_mixed_styles_consistent() {
        let lines = vec![
            "# Title\n",
            "\n",
            "```\n",
            "fenced code\n",
            "```\n",
            "\n",
            "    indented code\n",
        ];
        let config = HashMap::new();
        let params = make_params(&lines, &config);
        let errors = MD046.lint(&params);
        assert_eq!(errors.len(), 1, "Mixed styles should report indented block");
        assert_eq!(errors[0].line_number, 7);
        assert_eq!(
            errors[0].error_detail,
            Some("Expected: fenced; Actual: indented".to_string())
        );
    }

    #[test]
    fn test_md046_style_fenced() {
        // With style=fenced, indented blocks are errors even without fenced blocks
        let lines = vec!["# Title\n", "\n", "    indented code\n"];
        let mut config = HashMap::new();
        config.insert(
            "style".to_string(),
            serde_json::Value::String("fenced".to_string()),
        );
        let params = make_params(&lines, &config);
        let errors = MD046.lint(&params);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line_number, 3);
    }

    #[test]
    fn test_md046_style_indented() {
        // With style=indented, fenced blocks are errors even without indented blocks
        let lines = vec!["# Title\n", "\n", "```\n", "code\n", "```\n"];
        let mut config = HashMap::new();
        config.insert(
            "style".to_string(),
            serde_json::Value::String("indented".to_string()),
        );
        let params = make_params(&lines, &config);
        let errors = MD046.lint(&params);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line_number, 3);
    }

    #[test]
    fn test_md046_tilde_fenced() {
        let lines = vec!["~~~\n", "code\n", "~~~\n", "\n", "    indented\n"];
        let config = HashMap::new();
        let params = make_params(&lines, &config);
        let errors = MD046.lint(&params);
        assert_eq!(
            errors.len(),
            1,
            "Tilde fenced + indented should trigger mixed style error"
        );
        assert_eq!(errors[0].line_number, 5);
    }

    #[test]
    fn test_md046_no_code_blocks() {
        let lines = vec!["# Title\n", "\n", "Just a paragraph.\n"];
        let config = HashMap::new();
        let params = make_params(&lines, &config);
        let errors = MD046.lint(&params);
        assert_eq!(errors.len(), 0, "No code blocks should not trigger MD046");
    }

    #[test]
    fn test_md046_indented_needs_blank_before() {
        // 4-space indent immediately after a paragraph is NOT an indented code block
        let lines = vec!["# Title\n", "Some text\n", "    not code\n"];
        let config = HashMap::new();
        let params = make_params(&lines, &config);
        let errors = MD046.lint(&params);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_md046_multiple_blocks() {
        // Blank lines within an indented block don't end it, so a
        // non-indented paragraph is needed to separate two blocks.
        let lines = vec![
            "# Title\n",
            "\n",
            "    indented 1\n",
            "\n",
            "paragraph\n",
            "\n",
            "    indented 2\n",
            "\n",
            "```\n",
            "fenced\n",
            "```\n",
        ];
        let mut config = HashMap::new();
        config.insert(
            "style".to_string(),
            serde_json::Value::String("fenced".to_string()),
        );
        let params = make_params(&lines, &config);
        let errors = MD046.lint(&params);
        assert_eq!(errors.len(), 2, "Both indented blocks should be flagged");
    }

    #[test]
    fn test_md046_no_fix_info() {
        let lines = vec!["```\n", "code\n", "```\n", "\n", "    indented\n"];
        let config = HashMap::new();
        let params = make_params(&lines, &config);
        let errors = MD046.lint(&params);
        assert_eq!(errors.len(), 1);
        assert!(
            errors[0].fix_info.is_none(),
            "MD046 should not have fix_info"
        );
    }
}
