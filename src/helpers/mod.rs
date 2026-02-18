//! Helper utilities

/// Check if a string is a valid URL
pub fn is_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}

/// Check if a string is empty
pub fn is_empty_string(s: &str) -> bool {
    s.is_empty()
}

/// Detect line ending style
pub fn detect_line_ending(content: &str) -> &str {
    if content.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    }
}

/// Check if a trimmed line starts a code fence (``` or ~~~)
#[inline]
pub fn is_code_fence(trimmed: &str) -> bool {
    trimmed.starts_with("```") || trimmed.starts_with("~~~")
}

/// Convert a heading text string to a GitHub-style anchor ID.
///
/// Rules: lowercase, spaces and hyphens become hyphens (de-duplicated),
/// all other non-alphanumeric characters are dropped, leading/trailing
/// hyphens are trimmed.
///
/// This matches the algorithm used by GitHub-Flavored Markdown and is
/// shared by MD051 and the LSP rename/completion handlers.
///
/// # Examples
/// ```
/// assert_eq!(mkdlint::helpers::heading_to_anchor_id("Hello World"), "hello-world");
/// assert_eq!(mkdlint::helpers::heading_to_anchor_id("What's New?"), "whats-new");
/// ```
pub fn heading_to_anchor_id(text: &str) -> String {
    let lower = text.to_lowercase();
    let mut id = String::with_capacity(lower.len());
    let mut prev_hyphen = false;
    for ch in lower.chars() {
        if ch.is_alphanumeric() {
            id.push(ch);
            prev_hyphen = false;
        } else if (ch == ' ' || ch == '-') && !prev_hyphen {
            id.push('-');
            prev_hyphen = true;
        }
        // Skip other characters (punctuation, etc.)
    }
    id.trim_matches('-').to_string()
}

/// Split content into lines preserving line endings
pub fn split_lines(content: &str) -> Vec<String> {
    let line_ending = detect_line_ending(content);
    content.split(line_ending).map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_url() {
        assert!(is_url("https://example.com"));
        assert!(is_url("http://example.com"));
        assert!(!is_url("example.com"));
        assert!(!is_url("not a url"));
    }

    #[test]
    fn test_detect_line_ending() {
        assert_eq!(detect_line_ending("line1\nline2"), "\n");
        assert_eq!(detect_line_ending("line1\r\nline2"), "\r\n");
    }
}
