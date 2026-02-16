# mkdlint Roadmap

This document outlines the planned features and improvements for mkdlint.

## Current Status: v0.9.1

- ✅ 53 lint rules (full markdownlint parity)
- ✅ 84.9% auto-fix coverage (45/53 rules)
- ✅ LSP server with real-time diagnostics and code actions
- ✅ Interactive configuration wizard
- ✅ Watch mode with debounced file system notifications
- ✅ GitHub Action with SARIF Code Scanning support
- ✅ Multi-platform binary releases
- ✅ Output formatters (text, JSON, SARIF)
- ✅ Colored terminal output with source context
- ✅ Performance-optimized: zero-copy lines, static strings, conditional parsing

---

## ~~Option 1: LSP Server (v0.9.0)~~ ✅ DONE

Shipped in v0.9.0 with full diagnostics, code actions, config discovery, and editor support for VS Code, Neovim, Emacs, Helix, Zed, and Sublime.

---

## ~~Option 4: Performance Optimization~~ ✅ DONE

Shipped in v0.9.1 with:

- Conditional parser skip (~400µs savings when token rules disabled)
- `&'static str` in LintError (eliminates heap allocs per error)
- Zero-copy line splitting with `split_inclusive`
- Config reference sharing (eliminates HashMap clones)
- ~5-8% improvement on single-file benchmarks

---

## Option 1: Push Auto-Fix to 90%+ (v0.10.0)

**Status:** Next up
**Effort:** 6-10 hours
**Impact:** High - Moves toward v1.0 readiness

### Current Coverage

45/53 rules have auto-fix (84.9%). 8 rules remain:

| Rule      | Description                    | Fix Feasibility                            |
| --------- | ------------------------------ | ------------------------------------------ |
| **MD013** | Line length                    | Hard - requires intelligent word wrapping  |
| **MD033** | Inline HTML                    | Hard - can't safely remove arbitrary HTML  |
| **MD043** | Required heading structure     | Hard - can't generate missing content      |
| **MD046** | Code block style               | Medium - convert indented to fenced        |
| **MD051** | Link fragments should be valid | Hard - requires guessing correct fragment  |
| **MD054** | Link and image style           | Medium - convert between link styles       |
| **MD056** | Table column count             | Medium - add/remove columns                |
| **MD059** | Emphasis style in math         | Easy - change emphasis markers             |

### Realistic Targets

Add auto-fix to **MD046, MD054, MD059** (the medium/easy ones):

- **MD059**: Change `*` to `_` (or vice versa) in math contexts - straightforward
- **MD046**: Convert indented code to fenced (or vice versa) - medium complexity
- **MD054**: Convert link styles (inline to reference) - medium complexity

This would reach **48/53 = 90.6%** coverage.

### Success Criteria

- Coverage reaches 90%+
- All fixes roundtrip correctly
- All tests pass

---

## Option 2: GitHub Action Enhancements (v0.10.x)

**Status:** Planned
**Effort:** 8-12 hours
**Impact:** Medium - Improves CI/CD workflows

### Features

- **Incremental linting** - Only lint changed files in PRs
- **PR comments** - Post review comments on violations
- **Job summary** - Rich GitHub Actions summary with stats
- **Performance metrics** - Report linting time and file counts
- **Additional formats** - Checkstyle XML, JUnit XML

### Implementation

1. **Incremental Linting**
   - Integrate with `changed-files` action
   - Filter files to only .md/.markdown
   - Report only new violations

2. **PR Comments**
   - Use GitHub API to post review comments
   - Map line numbers to PR diff
   - Group by file and rule

3. **Enhanced Reporting**
   - Generate markdown tables for job summary
   - Include before/after stats for auto-fix
   - Performance timing

4. **Additional Formats**
   - Checkstyle XML format
   - JUnit XML format

---

## Option 3: Ecosystem Integration (v0.10.x)

**Status:** Planned
**Effort:** 8-16 hours
**Impact:** Medium - Broader adoption

### Integrations

1. **pre-commit Hooks**
   - Create .pre-commit-hooks.yaml
   - Publish to pre-commit.com
   - Documentation for setup

2. **Docker Image**
   - Official Docker image on Docker Hub
   - Multi-arch support (amd64, arm64)
   - Minimal Alpine-based image

3. **Homebrew Formula**
   - Create Homebrew tap
   - Auto-update on releases

4. **Rust Crate Publishing**
   - Publish library to crates.io
   - Comprehensive API docs
   - Example usage in docs

5. **npm Package**
   - Wrapper for Node.js projects
   - npx mkdlint support

---

## Option 4: VS Code Extension (v0.10.x)

**Status:** Planned
**Effort:** 10-14 hours
**Impact:** High - Primary editor for most users

### Extension Features

- Native VS Code extension wrapping the LSP server
- Bundled mkdlint-lsp binary
- Extension settings for common config options
- Status bar with error/warning counts
- Marketplace publishing

### Extension Implementation

1. **Extension scaffold** - TypeScript, vsce packaging
2. **LSP client** - Connect to bundled mkdlint-lsp
3. **Configuration** - Map VS Code settings to mkdlint config
4. **Packaging** - Bundle binaries for all platforms
5. **Marketplace** - Publish to VS Code Marketplace

---

## Long-Term Vision (v1.0+)

### v1.0 Release Criteria

- ✅ LSP server fully implemented and stable
- ✅ Performance benchmarks published
- 🔲 90%+ auto-fix coverage
- 🔲 Published on crates.io
- 🔲 VS Code extension on Marketplace
- 🔲 Comprehensive test suite (95%+ coverage)
- 🔲 Full documentation (user guide, API docs, tutorials)

### Beyond v1.0

- **Custom Rules API** - Plugin system for user-defined rules
- **Markdown Formatter** - Full document reformatting
- **Multi-language support** - MDX, AsciiDoc, reStructuredText

---

## Priority Matrix

| Feature                    | Impact | Effort | Priority |
| -------------------------- | ------ | ------ | -------- |
| ~~LSP Server~~             | High   | High   | Done     |
| ~~Performance~~            | Medium | Medium | Done     |
| Auto-Fix to 90%+           | High   | Medium | P1 Next  |
| VS Code Extension          | High   | Medium | P1       |
| GitHub Action Enhancements | Medium | Medium | P2       |
| Ecosystem Integration      | Medium | Medium | P2       |

---

## Contributing

Want to help with any of these features? Check out:

- [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines
- [GitHub Issues](https://github.com/192d-Wing/mkdlint/issues) for current work
- [Discussions](https://github.com/192d-Wing/mkdlint/discussions) for feature requests

---

## Feedback

Have ideas for the roadmap? Open a discussion or issue on GitHub!

**Last Updated:** 2026-02-16
**Current Version:** v0.9.1
