# mkdlint Roadmap

This document outlines the planned features and improvements for mkdlint.

## Current Status: v0.8.1

- ✅ 81.5% auto-fix coverage (44/54 rules)
- ✅ Interactive configuration wizard
- ✅ Watch mode with debounced file system notifications
- ✅ Comprehensive IDE integration documentation
- ✅ GitHub Action with SARIF Code Scanning support
- ✅ Multi-platform binary releases

---

## Option 1: LSP Server (v0.9.0) 🎯 HIGH PRIORITY

**Status:** Planned
**Effort:** 16-22 hours
**Impact:** High - Enables real-time editor integration

### Goals

Implement a full-featured Language Server Protocol server for real-time linting in editors.

### Features

- **Real-time diagnostics** - Lint as you type with debounced updates
- **Code actions** - Quick-fix integration for all auto-fixable rules
- **Workspace awareness** - Config discovery and multi-file support
- **Editor support** - VS Code, Neovim, Emacs, Zed, Sublime, etc.
- **Performance** - Efficient caching and incremental updates

### Implementation Components

1. **Core LSP Backend** (`src/lsp/backend.rs`)
   - LanguageServer trait implementation
   - Document lifecycle handlers (open, change, save, close)
   - Configuration management
   - Workspace initialization

2. **Diagnostics** (`src/lsp/diagnostics.rs`)
   - Convert LintError → LSP Diagnostic
   - Range calculation with UTF-8 support
   - Severity mapping
   - Related information

3. **Code Actions** (`src/lsp/code_actions.rs`)
   - Convert FixInfo → LSP TextEdit
   - Quick-fix actions for all auto-fixable rules
   - WorkspaceEdit generation

4. **Document Management** (`src/lsp/document.rs`)
   - In-memory document cache
   - Version tracking
   - Cached lint results

5. **Config Discovery** (`src/lsp/config.rs`)
   - Walk up directory tree to find config
   - Cache configs by directory
   - Invalidate on config file changes

6. **Utilities** (`src/lsp/utils.rs`)
   - Debouncer with tokio
   - Position/Range helpers
   - URI ↔ PathBuf conversion

7. **Binary** (`src/bin/mkdlint-lsp.rs`)
   - LSP server entry point
   - stdio transport
   - Logging to stderr

### Dependencies

- `tower-lsp` v0.20
- `lsp-types` v0.97
- `tokio` (extend existing with rt-multi-thread, fs, io-util, sync, time)
- `tower` v0.5

### Testing

- Unit tests for each module
- Integration tests for LSP lifecycle
- Manual testing with VS Code, Neovim

### Success Criteria

- ✅ Real-time diagnostics in VS Code
- ✅ Code actions apply fixes correctly
- ✅ Config discovery works across workspaces
- ✅ Debouncing prevents excessive re-linting
- ✅ All tests pass

---

## Option 2: GitHub Action Enhancements (v0.8.2)

**Status:** Planned
**Effort:** 8-12 hours
**Impact:** Medium - Improves CI/CD workflows

### Features

- **Incremental linting** - Only lint changed files in PRs
- **PR comments** - Post review comments on violations
- **Job summary** - Rich GitHub Actions summary with stats
- **Performance metrics** - Report linting time and file counts
- **Custom reporters** - Checkstyle, JUnit XML formats
- **Binary checksum verification** - Verify downloaded binaries

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
   - Add emoji indicators and progress bars

4. **Additional Formats**
   - Checkstyle XML format
   - JUnit XML format
   - ESLint JSON format

### Success Criteria

- ✅ Incremental linting works on PRs
- ✅ Comments appear on correct PR lines
- ✅ Job summary shows rich statistics
- ✅ All output formats validated

---

## Option 3: Push Auto-Fix to 85%+ (v0.8.3)

**Status:** Planned
**Effort:** 10-14 hours
**Impact:** Medium-High - Reduces manual fixes

### Target

Add auto-fix support to 3-5 more rules, reaching 47-49/54 (87-91% coverage).

### Candidate Rules

**High Priority (Straightforward):**

1. **MD003** - Heading style consistency
   - Convert ATX ↔ Setext based on config
   - Normalize heading markers (# vs ##)

2. **MD020** - Spaces inside hashes on closed ATX headings
   - Insert space after opening #
   - Insert space before closing #

3. **MD021** - Multiple spaces inside hashes on closed ATX headings
   - Delete extra spaces (keep only 1)

4. **MD014** - Dollar signs used before commands
   - Remove $ or $ prefix from code blocks

**Medium Priority (More Complex):**

5. **MD028** - Blank line inside blockquote
   - Delete blank lines within blockquotes

6. **MD036** - Emphasis used instead of heading
   - Convert **text** or *text* to ## text

7. **MD042** - No empty links
   - Fill empty URLs with placeholder (#link)

**Lower Priority (Requires Heuristics):**

8. **MD013** - Line length (requires intelligent wrapping)
9. **MD046** - Code block style (complex token manipulation)

### Implementation Strategy

Follow existing patterns from MD023, MD026, MD034:
- Add FixInfo to error generation
- Calculate edit_column (1-based)
- Set delete_count and insert_text
- Add "fixable" tag
- Write 2-3 tests per rule

### Success Criteria

- ✅ Coverage reaches 85%+
- ✅ All fixes roundtrip correctly
- ✅ All tests pass
- ✅ Zero clippy warnings

---

## Option 4: Performance Optimization (v0.8.4)

**Status:** Planned
**Effort:** 6-10 hours
**Impact:** Medium - Faster execution

### Goals

Profile and optimize mkdlint for even faster linting, especially on large codebases.

### Areas to Optimize

1. **Parallel Processing**
   - Use rayon for parallel file processing
   - Benchmark single-threaded vs multi-threaded
   - Optimal thread pool sizing

2. **Parser Caching**
   - Cache parse results for unchanged files
   - Use file modification time for cache invalidation
   - LRU cache for frequently accessed files

3. **Rule Execution**
   - Benchmark each rule's performance
   - Optimize hot paths in expensive rules
   - Consider rule dependencies for early termination

4. **Memory Usage**
   - Profile heap allocations
   - Reduce cloning where possible
   - Use Cow<str> for zero-copy operations

5. **I/O Optimization**
   - Batch file reads
   - Memory-mapped files for very large documents
   - Async I/O for concurrent file access

### Benchmarking

- Add comprehensive benchmarks (already have benches/lint_bench.rs)
- Test on repos of varying sizes (10, 100, 1000+ files)
- Measure both warm and cold cache performance

### Success Criteria

- ✅ 20%+ speedup on large codebases (100+ files)
- ✅ Memory usage remains stable
- ✅ Benchmarks show measurable improvements
- ✅ No regressions in functionality

---

## Option 5: Advanced Features (v0.9.x)

**Status:** Planned
**Effort:** Varies
**Impact:** Medium - Nice-to-have features

### Potential Features

1. **Custom Rules API**
   - Plugin system for user-defined rules
   - Trait-based rule definition
   - Dynamic rule loading

2. **Machine Learning Suggestions**
   - Analyze patterns in markdown files
   - Suggest style improvements
   - Learn from user fixes

3. **Markdown Formatter**
   - Full document reformatting
   - Preserve semantic meaning
   - Configurable style preferences

4. **Live Preview Integration**
   - Preview markdown with violations highlighted
   - Side-by-side before/after view
   - Integration with markdown preview tools

5. **Team Analytics**
   - Track violation trends over time
   - Most common issues by team/repo
   - Dashboard for markdown health

6. **VS Code Extension**
   - Native VS Code extension (separate from LSP)
   - Custom UI for rule management
   - Inline previews and suggestions

---

## Option 6: Ecosystem Integration (v0.9.x)

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
   - cask for GUI tools

4. **npm Package**
   - Wrapper for Node.js projects
   - npx mkdlint support
   - package.json scripts integration

5. **Rust Crate Publishing**
   - Publish library to crates.io
   - Comprehensive API docs
   - Example usage in docs

6. **Editor Plugins**
   - Neovim plugin (separate from LSP)
   - Emacs package
   - IntelliJ IDEA plugin

---

## Long-Term Vision (v1.0+)

### v1.0 Release Criteria

- ✅ LSP server fully implemented and stable
- ✅ 90%+ auto-fix coverage
- ✅ Comprehensive test suite (95%+ coverage)
- ✅ Full documentation (user guide, API docs, tutorials)
- ✅ Production usage in 100+ repositories
- ✅ Zero known critical bugs
- ✅ Performance benchmarks published

### Beyond v1.0

- **Multi-language support** - Support for MDX, AsciiDoc, reStructuredText
- **Cloud service** - SaaS for markdown linting with API
- **Enterprise features** - Team management, SSO, audit logs
- **AI-powered suggestions** - GPT-based style improvements
- **Collaborative features** - Shared configs, team dashboards

---

## Priority Matrix

| Feature | Impact | Effort | Priority |
|---------|--------|--------|----------|
| LSP Server | High | High | P0 🎯 |
| Auto-Fix to 85%+ | Medium-High | Medium | P1 |
| Performance Optimization | Medium | Medium | P2 |
| GitHub Action Enhancements | Medium | Medium | P2 |
| Advanced Features | Medium | High | P3 |
| Ecosystem Integration | Medium | Medium | P3 |

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
**Current Version:** v0.8.1
