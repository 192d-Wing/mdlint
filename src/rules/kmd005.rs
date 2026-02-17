//! KMD005 - No duplicate heading IDs
//!
//! In Kramdown, each heading gets an ID either from an explicit IAL (`{#id}`)
//! or from an auto-generated slug. Duplicate IDs break anchor navigation and
//! are invalid HTML.
//!
//! Auto-slug algorithm (matches Kramdown): lowercase the heading text, replace
//! spaces with hyphens, strip all non-alphanumeric-or-hyphen characters.

use crate::types::{LintError, ParserType, Rule, RuleParams, Severity};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

/// Matches ATX headings (with optional trailing IAL): `## Title {#custom-id}`
static ATX_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(#{1,6})\s+(.+?)(?:\s*\{[^}]*\})?\s*$").unwrap());

/// Matches an explicit `{#id}` attribute in an IAL or inline heading suffix
static EXPLICIT_ID_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{[^}]*#([A-Za-z][\w-]*)[^}]*\}").unwrap());

/// Generate a Kramdown-style heading slug from heading text.
///
/// Algorithm: lowercase, keep alphanumeric + hyphens, replace spaces with `-`,
/// strip everything else, collapse multiple hyphens.
fn kramdown_slug(text: &str) -> String {
    // Strip any trailing IAL from the text first
    let text = if let Some(pos) = text.rfind('{') {
        if text[pos..].ends_with('}') {
            text[..pos].trim()
        } else {
            text
        }
    } else {
        text
    };

    let mut slug = String::with_capacity(text.len());
    for ch in text.chars() {
        if ch.is_alphanumeric() {
            for c in ch.to_lowercase() {
                slug.push(c);
            }
        } else if ch == ' ' || ch == '-' {
            slug.push('-');
        }
        // All other chars are stripped
    }

    // Collapse multiple consecutive hyphens
    let re = Regex::new(r"-{2,}").unwrap();
    let slug = re.replace_all(&slug, "-").into_owned();
    slug.trim_matches('-').to_string()
}

pub struct KMD005;

impl Rule for KMD005 {
    fn names(&self) -> &'static [&'static str] {
        &["KMD005", "no-duplicate-heading-ids"]
    }

    fn description(&self) -> &'static str {
        "Heading IDs must be unique within the document"
    }

    fn tags(&self) -> &[&'static str] {
        &["kramdown", "headings", "ids"]
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

        // id → first line number
        let mut seen: HashMap<String, usize> = HashMap::new();
        let mut in_code_block = false;

        for (idx, line) in lines.iter().enumerate() {
            let line_number = idx + 1;
            let trimmed = line.trim_end_matches('\n').trim_end_matches('\r');

            // Track code fences
            if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                in_code_block = !in_code_block;
                continue;
            }
            if in_code_block {
                continue;
            }

            // Only process ATX headings
            if let Some(cap) = ATX_RE.captures(trimmed) {
                let heading_text = cap[2].trim();

                // Determine the heading ID: explicit takes priority
                let id = if let Some(explicit) = EXPLICIT_ID_RE.captures(trimmed) {
                    explicit[1].to_string()
                } else {
                    kramdown_slug(heading_text)
                };

                if id.is_empty() {
                    continue;
                }

                if let Some(&first_line) = seen.get(&id) {
                    errors.push(LintError {
                        line_number,
                        rule_names: self.names(),
                        rule_description: self.description(),
                        error_detail: Some(format!(
                            "Duplicate heading ID '{id}' (first defined on line {first_line})"
                        )),
                        severity: Severity::Error,
                        ..Default::default()
                    });
                } else {
                    seen.insert(id, line_number);
                }
            }
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
        let rule = KMD005;
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
    fn test_kmd005_unique_headings_ok() {
        let errors = lint("# Intro\n\n## Setup\n\n## Usage\n");
        assert!(errors.is_empty(), "unique headings should not fire");
    }

    #[test]
    fn test_kmd005_duplicate_auto_slug() {
        let errors = lint("# Setup\n\n## Setup\n");
        assert!(
            errors.iter().any(|e| e.rule_names[0] == "KMD005"),
            "should fire when two headings produce the same auto-slug"
        );
    }

    #[test]
    fn test_kmd005_explicit_id_duplicate() {
        let errors = lint("# Title {#intro}\n\n## Other {#intro}\n");
        assert!(
            errors.iter().any(|e| e.rule_names[0] == "KMD005"),
            "should fire when two headings share an explicit ID"
        );
    }

    #[test]
    fn test_kmd005_kramdown_slug_generation() {
        assert_eq!(kramdown_slug("Hello World"), "hello-world");
        assert_eq!(kramdown_slug("Setup & Config!"), "setup-config");
        assert_eq!(kramdown_slug("  Leading spaces  "), "leading-spaces");
    }
}
