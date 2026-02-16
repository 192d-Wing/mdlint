# I-PLAN: Complete and Test LSP Server Implementation

**Status:** 90% Implemented - Needs testing, documentation, and minor enhancements
**Target Version:** v0.9.0
**Estimated Effort:** 6-10 hours (reduced from 16-22 due to existing implementation)

---

## 1. Current State Analysis

### ✅ What's Already Implemented

**Core LSP Infrastructure (~1,164 lines)**

1. **`src/lsp/backend.rs`** (346 lines) ✅
   - Full LanguageServer trait implementation
   - Document lifecycle handlers (open, change, save, close)
   - Code actions with individual fixes
   - Execute command handler for "Fix All"
   - Debounced linting on changes
   - Proper initialization/shutdown

2. **`src/lsp/document.rs`** (190 lines) ✅
   - DocumentManager with DashMap for thread-safe storage
   - Document struct with content, version, cached_errors
   - Methods: insert, get, update, remove, update_errors

3. **`src/lsp/diagnostics.rs`** (155 lines) ✅
   - lint_error_to_diagnostic conversion
   - Range calculation from error_range or full line
   - Severity mapping (Error → ERROR, Warning → WARNING)
   - Message formatting with rule names and details

4. **`src/lsp/code_actions.rs`** (274 lines) ✅
   - fix_to_code_action for individual fixes
   - TextEdit generation from FixInfo
   - Handles delete_count = -1 (delete line), > 0 (delete chars), None (insert only)
   - WorkspaceEdit creation

5. **`src/lsp/utils.rs`** (163 lines) ✅
   - Debouncer implementation with tokio
   - Schedule and cancel operations
   - 300ms default debounce delay

6. **`src/lsp/mod.rs`** (36 lines) ✅
   - Module organization
   - Public re-exports (MkdlintLanguageServer)

7. **`src/bin/mkdlint-lsp.rs`** (26 lines) ✅
   - LSP server binary entry point
   - stdio transport setup
   - Logging to stderr

8. **`Cargo.toml`** ✅
   - All dependencies configured (tower-lsp 0.20, lsp-types 0.94, etc.)
   - `lsp` feature flag properly set up
   - Binary target configured with required-features

9. **`src/lib.rs`** ✅
   - `pub mod lsp;` export

### ❌ What's Missing

1. **Tests** (0 lines) - Critical Gap
   - No unit tests for any LSP modules
   - No integration tests for LSP lifecycle
   - No manual testing documentation

2. **Config Discovery** - Not Implemented
   - No config.rs module
   - No workspace root tracking
   - No config auto-discovery walking up directory tree
   - No config caching or invalidation

3. **Documentation** - Minimal
   - No user guide for LSP setup
   - No editor integration examples (VS Code, Neovim, etc.)
   - Only basic doc comments in code

4. **Performance Optimization** - Not Implemented
   - No incremental parsing
   - No workspace-wide diagnostics
   - No file watching for config changes

5. **Missing LSP Features** - Optional but useful
   - No hover information
   - No formatting provider
   - No document symbols
   - No workspace symbols

---

## 2. Implementation Plan

### Phase 1: Add Config Discovery (2-3 hours)

**Create `src/lsp/config.rs`** (~150 lines)

```rust
//! Configuration discovery and caching for LSP

use crate::config::Config;
use dashmap::DashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tower_lsp::lsp_types::Url;

/// Manages configuration discovery and caching
pub struct ConfigManager {
    /// Cache of configs by directory path
    cache: Arc<DashMap<PathBuf, Option<Config>>>,
    /// Workspace roots
    workspace_roots: Vec<PathBuf>,
}

impl ConfigManager {
    pub fn new(workspace_roots: Vec<PathBuf>) -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
            workspace_roots,
        }
    }

    /// Discover config for a file URI
    pub fn discover_config(&self, uri: &Url) -> Option<Config> {
        let file_path = uri.to_file_path().ok()?;
        let dir = file_path.parent()?;
        
        // Check cache first
        if let Some(entry) = self.cache.get(dir) {
            return entry.clone();
        }

        // Walk up directory tree to workspace root
        let config = self.find_config(dir);
        
        // Cache result
        self.cache.insert(dir.to_path_buf(), config.clone());
        
        config
    }

    /// Walk up directory tree looking for config files
    fn find_config(&self, start_dir: &Path) -> Option<Config> {
        let mut current = start_dir;
        
        loop {
            // Try known config file names
            for name in &[
                ".markdownlint.json",
                ".markdownlint.yaml",
                ".markdownlint.yml",
                ".markdownlintrc",
            ] {
                let config_path = current.join(name);
                if config_path.exists() {
                    if let Ok(config) = Config::from_file(&config_path) {
                        return Some(config);
                    }
                }
            }

            // Stop at workspace root
            if self.workspace_roots.iter().any(|root| current == root) {
                break;
            }

            // Go up one level
            current = current.parent()?;
        }

        None
    }

    /// Invalidate cache for a directory (when config changes)
    pub fn invalidate(&self, path: &Path) {
        self.cache.remove(path);
    }

    /// Clear entire cache
    pub fn clear_cache(&self) {
        self.cache.clear();
    }
}
```

**Update `src/lsp/backend.rs`**:
- Add `config_manager: Arc<ConfigManager>` field
- Store workspace roots from initialize params
- Pass config to lint_sync options
- Handle did_change_watched_files for config files

**Tests** (`src/lsp/config.rs` bottom):
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_config_discovery() {
        // Create temp workspace
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        
        // Create nested structure
        let subdir = root.join("docs/guides");
        fs::create_dir_all(&subdir).unwrap();
        
        // Create config at root
        let config_path = root.join(".markdownlint.json");
        fs::write(&config_path, r#"{"MD013": false}"#).unwrap();
        
        // Test discovery
        let manager = ConfigManager::new(vec![root.to_path_buf()]);
        let config = manager.find_config(&subdir);
        assert!(config.is_some());
    }

    #[test]
    fn test_config_caching() {
        let temp = TempDir::new().unwrap();
        let manager = ConfigManager::new(vec![temp.path().to_path_buf()]);
        
        // First call
        let config1 = manager.find_config(temp.path());
        
        // Second call should hit cache
        let config2 = manager.find_config(temp.path());
        
        // Results should be identical
        assert_eq!(config1.is_some(), config2.is_some());
    }

    #[test]
    fn test_cache_invalidation() {
        let temp = TempDir::new().unwrap();
        let manager = ConfigManager::new(vec![temp.path().to_path_buf()]);
        
        // Populate cache
        let _ = manager.find_config(temp.path());
        assert_eq!(manager.cache.len(), 1);
        
        // Invalidate
        manager.invalidate(temp.path());
        
        // Should be gone (or re-computed on next access)
        assert_eq!(manager.cache.len(), 0);
    }
}
```

---

### Phase 2: Comprehensive Testing (3-4 hours)

**Create `tests/lsp_integration.rs`** (~400 lines)

```rust
#![cfg(feature = "lsp")]

use mkdlint::lsp::MkdlintLanguageServer;
use tower_lsp::{LspService, jsonrpc::Response};
use tower_lsp::lsp_types::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn test_initialize_and_shutdown() {
    let (service, socket) = LspService::new(MkdlintLanguageServer::new);
    
    // Send initialize request
    let init_params = InitializeParams {
        capabilities: ClientCapabilities::default(),
        ..Default::default()
    };
    
    let result = service.inner().initialize(init_params).await.unwrap();
    
    // Verify capabilities
    assert!(result.capabilities.text_document_sync.is_some());
    assert!(result.capabilities.code_action_provider.is_some());
    
    // Shutdown
    service.inner().shutdown().await.unwrap();
}

#[tokio::test]
async fn test_did_open_publishes_diagnostics() {
    let (service, socket) = LspService::new(MkdlintLanguageServer::new);
    let server = service.inner();
    
    // Initialize
    server.initialize(InitializeParams::default()).await.unwrap();
    server.initialized(InitializedParams {}).await;
    
    // Open document with issues
    let uri = Url::parse("file:///test.md").unwrap();
    let content = "#No space after hash\n\nTrailing spaces:   \n".to_string();
    
    server.did_open(DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "markdown".to_string(),
            version: 1,
            text: content,
        },
    }).await;
    
    // Wait for async processing
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // Verify document is stored
    // (Need access to document_manager - may need to make it public for testing)
}

#[tokio::test]
async fn test_code_actions_for_fixable_errors() {
    let (service, socket) = LspService::new(MkdlintLanguageServer::new);
    let server = service.inner();
    
    server.initialize(InitializeParams::default()).await.unwrap();
    server.initialized(InitializedParams {}).await;
    
    // Open document
    let uri = Url::parse("file:///test.md").unwrap();
    let content = "#Bad\nTrailing:   \n".to_string();
    
    server.did_open(DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "markdown".to_string(),
            version: 1,
            text: content,
        },
    }).await;
    
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // Request code actions
    let actions = server.code_action(CodeActionParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        range: Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 2, character: 0 },
        },
        context: CodeActionContext {
            diagnostics: vec![],
            only: None,
            trigger_kind: None,
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    }).await.unwrap();
    
    // Should have at least "Fix All" action
    assert!(actions.is_some());
    let actions = actions.unwrap();
    assert!(!actions.is_empty());
}

#[tokio::test]
async fn test_debouncing_rapid_changes() {
    let (service, socket) = LspService::new(MkdlintLanguageServer::new);
    let server = service.inner();
    
    server.initialize(InitializeParams::default()).await.unwrap();
    server.initialized(InitializedParams {}).await;
    
    let uri = Url::parse("file:///test.md").unwrap();
    
    // Open document
    server.did_open(DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "markdown".to_string(),
            version: 1,
            text: "# Test\n".to_string(),
        },
    }).await;
    
    // Make rapid changes
    for i in 2..10 {
        server.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: i,
            },
            content_changes: vec![TextDocumentContentChangeEvent {
                range: None,
                range_length: None,
                text: format!("# Test {}\n", i),
            }],
        }).await;
        
        // Small delay between changes
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
    
    // Wait for debounce to settle
    tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;
    
    // Only last change should have been linted (debounced)
}

#[tokio::test]
async fn test_fix_all_command() {
    let (service, socket) = LspService::new(MkdlintLanguageServer::new);
    let server = service.inner();
    
    server.initialize(InitializeParams::default()).await.unwrap();
    server.initialized(InitializedParams {}).await;
    
    let uri = Url::parse("file:///test.md").unwrap();
    let content = "#Bad\n#AlsoBad\n".to_string();
    
    server.did_open(DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "markdown".to_string(),
            version: 1,
            text: content,
        },
    }).await;
    
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // Execute fixAll command
    let result = server.execute_command(ExecuteCommandParams {
        command: "mkdlint.fixAll".to_string(),
        arguments: vec![serde_json::to_value(&uri).unwrap()],
        work_done_progress_params: WorkDoneProgressParams::default(),
    }).await;
    
    assert!(result.is_ok());
}
```

**Unit tests for each module** (add to bottom of each file):

1. **`src/lsp/diagnostics.rs`** - 4 tests:
   - `test_error_to_diagnostic_with_range()`
   - `test_error_to_diagnostic_full_line()`
   - `test_severity_mapping()`
   - `test_utf8_handling()`

2. **`src/lsp/code_actions.rs`** - 4 tests:
   - `test_fix_delete_characters()`
   - `test_fix_insert_text()`
   - `test_fix_delete_line()`
   - `test_fix_replace()`

3. **`src/lsp/document.rs`** - 3 tests:
   - `test_document_lifecycle()`
   - `test_concurrent_access()`
   - `test_error_caching()`

4. **`src/lsp/utils.rs`** - 2 tests:
   - `test_debouncer_schedule()`
   - `test_debouncer_cancel()`

---

### Phase 3: Documentation and Editor Integration (2-3 hours)

**Update `docs/USER_GUIDE.md`** - Add LSP section (~300 lines):

```markdown
## Language Server Protocol (LSP)

mkdlint provides a full-featured LSP server for real-time linting in editors.

### Features

- ✅ Real-time diagnostics as you type
- ✅ Code actions (quick fixes) for all 44 auto-fixable rules
- ✅ "Fix All" command to apply all fixes at once
- ✅ Debounced updates (300ms delay)
- ✅ Config auto-discovery (.markdownlint.json, etc.)
- ✅ Workspace support

### Installation

The LSP server is included when you install mkdlint:

```bash
# From source
cargo install mkdlint --features lsp

# Binary should be at:
which mkdlint-lsp
```

### VS Code Setup

1. Install a generic LSP client (or create custom extension later)

**Using vscode-languageclient:**

Create `.vscode/settings.json`:

```json
{
  "mkdlint.enable": true,
  "mkdlint.server.path": "/path/to/mkdlint-lsp",
  "mkdlint.trace.server": "verbose"
}
```

**OR using a simple client:**

```typescript
import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  const serverOptions: ServerOptions = {
    command: 'mkdlint-lsp',
    args: [],
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'markdown' }],
  };

  client = new LanguageClient(
    'mkdlint',
    'mkdlint LSP',
    serverOptions,
    clientOptions
  );

  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
```

### Neovim Setup

Using `nvim-lspconfig`:

```lua
local lspconfig = require('lspconfig')
local configs = require('lspconfig.configs')

-- Define mkdlint LSP config
if not configs.mkdlint then
  configs.mkdlint = {
    default_config = {
      cmd = { 'mkdlint-lsp' },
      filetypes = { 'markdown' },
      root_dir = lspconfig.util.root_pattern('.markdownlint.json', '.git'),
      settings = {},
    },
  }
end

-- Setup
lspconfig.mkdlint.setup({
  on_attach = function(client, bufnr)
    -- Enable completion
    vim.api.nvim_buf_set_option(bufnr, 'omnifunc', 'v:lua.vim.lsp.omnifunc')

    -- Keybindings
    local opts = { noremap=true, silent=true, buffer=bufnr }
    vim.keymap.set('n', 'gd', vim.lsp.buf.definition, opts)
    vim.keymap.set('n', 'K', vim.lsp.buf.hover, opts)
    vim.keymap.set('n', '<leader>ca', vim.lsp.buf.code_action, opts)
    
    -- Format on save
    if client.server_capabilities.documentFormattingProvider then
      vim.api.nvim_create_autocmd("BufWritePre", {
        buffer = bufnr,
        callback = function()
          vim.lsp.buf.format({ async = false })
        end,
      })
    end
  end,
})
```

### Emacs Setup

Using `lsp-mode`:

```elisp
(use-package lsp-mode
  :hook ((markdown-mode . lsp))
  :commands lsp
  :config
  (lsp-register-client
   (make-lsp-client
    :new-connection (lsp-stdio-connection "mkdlint-lsp")
    :major-modes '(markdown-mode)
    :server-id 'mkdlint)))
```

### Helix Setup

Add to `~/.config/helix/languages.toml`:

```toml
[[language]]
name = "markdown"
language-server = { command = "mkdlint-lsp" }
```

### Troubleshooting

**LSP not starting:**
- Check `mkdlint-lsp --version` works
- Check editor LSP logs (VS Code: Output → mkdlint)
- Enable verbose logging: `RUST_LOG=debug mkdlint-lsp`

**No diagnostics appearing:**
- Ensure file is saved (some editors need save to trigger)
- Check file is .md or .markdown extension
- Check no config errors in .markdownlint.json

**Code actions not working:**
- Only fixable rules show code actions
- Check diagnostic is selected/cursor is on line
- Try "Fix All" command

**Performance issues:**
- Increase debounce delay (future config option)
- Disable certain expensive rules in config
- Check file size (very large files may be slow)
```

**Create `docs/LSP.md`** (~200 lines):
- Architecture overview
- Protocol messages supported
- Extension points for future features
- Performance characteristics

**Update `README.md`**:
- Add LSP to features list
- Add quick setup example
- Link to LSP documentation

---

### Phase 4: Polish and Release Preparation (1-2 hours)

**Enhancements to existing code:**

1. **Expose document_manager for testing** (`src/lsp/backend.rs`):
   ```rust
   #[cfg(test)]
   pub fn get_document_manager(&self) -> &DocumentManager {
       &self.document_manager
   }
   ```

2. **Add workspace symbol support** (optional, nice-to-have):
   ```rust
   async fn document_symbol(
       &self,
       params: DocumentSymbolParams,
   ) -> Result<Option<DocumentSymbolResponse>> {
       // Return list of headings as symbols
   }
   ```

3. **Add formatting provider** (optional, uses apply_fixes):
   ```rust
   async fn formatting(
       &self,
       params: DocumentFormattingParams,
   ) -> Result<Option<Vec<TextEdit>>> {
       // Return edits that apply all fixes
   }
   ```

4. **Improve error messages**:
   - Add more context to log messages
   - Better error recovery (don't crash on malformed docs)

5. **Performance logging**:
   ```rust
   let start = std::time::Instant::now();
   // ... lint ...
   let elapsed = start.elapsed();
   self.client.log_message(
       MessageType::LOG,
       format!("Linted {} in {:?}", file_name, elapsed)
   ).await;
   ```

**Update CHANGELOG.md**:
```markdown
## [0.9.0] - 2026-02-XX

### Added

- **Language Server Protocol (LSP) Support** 🚀
  - Full-featured LSP server (`mkdlint-lsp`) for real-time editor integration
  - Real-time diagnostics with 300ms debounced updates
  - Code actions (quick fixes) for all 44 auto-fixable rules
  - "Fix All" command to apply all fixes at once
  - Automatic config discovery (.markdownlint.json, .yaml, etc.)
  - Workspace-aware with root detection
  - Tested with VS Code, Neovim, Emacs, Helix

### Technical

- New `src/lsp/` module with 1,300+ lines
- Integration tests for LSP lifecycle
- Config auto-discovery with caching
- Documentation for editor setup
```

**Update README.md**:
```markdown
## Features

- 🚀 **Language Server Protocol** - Real-time linting in VS Code, Neovim, Emacs, and more
```

---

## 3. Testing Plan

### Unit Tests (20+ tests)

1. **Config Discovery** (3 tests)
   - ✅ Finds config in parent directories
   - ✅ Stops at workspace root
   - ✅ Caches results

2. **Diagnostics** (4 tests)
   - ✅ Converts errors with ranges
   - ✅ Converts errors without ranges (full line)
   - ✅ Maps severity correctly
   - ✅ Handles UTF-8 properly

3. **Code Actions** (4 tests)
   - ✅ Generates delete edits
   - ✅ Generates insert edits
   - ✅ Generates replace edits
   - ✅ Handles delete line (-1)

4. **Document Manager** (3 tests)
   - ✅ CRUD operations
   - ✅ Thread safety
   - ✅ Error caching

5. **Debouncer** (2 tests)
   - ✅ Schedules tasks
   - ✅ Cancels pending tasks

### Integration Tests (5 tests)

1. ✅ Initialize and shutdown
2. ✅ Open document publishes diagnostics
3. ✅ Code actions for fixable errors
4. ✅ Debouncing rapid changes
5. ✅ Fix All command execution

### Manual Testing

1. **VS Code**:
   - Open .md file → see diagnostics
   - Edit file → debounced updates
   - Click quick fix → applies fix
   - Run "Fix All" → all fixes applied

2. **Neovim**:
   - Same as VS Code
   - Verify keybindings work

3. **Performance**:
   - Large file (10,000+ lines) → reasonable speed
   - Multiple files open → no slowdown
   - Rapid typing → debouncing works

---

## 4. Success Criteria

### Must Have ✅

- [ ] Config discovery implemented and tested
- [ ] All unit tests passing (20+ tests)
- [ ] All integration tests passing (5 tests)
- [ ] Works in VS Code with diagnostics and code actions
- [ ] Works in Neovim with diagnostics and code actions
- [ ] Documentation complete (USER_GUIDE.md, LSP.md)
- [ ] CHANGELOG updated
- [ ] README updated
- [ ] Zero compilation warnings

### Nice to Have 🎯

- [ ] Formatting provider (uses apply_fixes)
- [ ] Document symbols (headings)
- [ ] Hover information (show rule docs)
- [ ] Performance logging
- [ ] Workspace diagnostics
- [ ] File watching for config changes

### Future Enhancements 🔮

- [ ] Incremental parsing
- [ ] Pull diagnostics model (LSP 3.17+)
- [ ] Inline values/hints
- [ ] Semantic tokens
- [ ] Custom configuration from client
- [ ] VS Code extension (separate repo)

---

## 5. Timeline

**Total: 6-10 hours** (90% already done!)

| Phase | Description | Time | Completion |
|-------|-------------|------|------------|
| 1 | Config Discovery | 2-3h | 0% |
| 2 | Testing | 3-4h | 0% |
| 3 | Documentation | 2-3h | 0% |
| 4 | Polish & Release | 1-2h | 0% |

**Breakdown:**
- Day 1 (4h): Phase 1 (Config) + Start Phase 2 (Tests)
- Day 2 (4h): Finish Phase 2 (Tests) + Phase 3 (Docs)
- Day 3 (2h): Phase 4 (Polish) + Release

---

## 6. Dependencies

**Already Satisfied:**
- ✅ tower-lsp 0.20
- ✅ lsp-types 0.94
- ✅ tokio with all required features
- ✅ tower 0.5
- ✅ url 2.5
- ✅ env_logger 0.11
- ✅ dashmap 6.1

**New Dependencies:**
- None! All dependencies already in Cargo.toml

---

## 7. Risks and Mitigations

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| LSP client compatibility issues | Medium | Low | Test with multiple editors (VS Code, Neovim, Emacs) |
| Config discovery edge cases | Low | Medium | Comprehensive unit tests, handle errors gracefully |
| Performance on large files | Medium | Low | Profile and optimize, add debouncing safeguards |
| Breaking API changes | Low | Low | LSP is stable, tower-lsp handles protocol details |

---

## 8. Next Steps

**Immediate (Start Here):**

1. Create `src/lsp/config.rs` with ConfigManager
2. Update `src/lsp/backend.rs` to use ConfigManager
3. Add config discovery tests

**Then:**

4. Write all unit tests for existing modules
5. Write integration tests
6. Manual testing with VS Code
7. Write documentation
8. Update CHANGELOG and README
9. Release v0.9.0

---

## 9. Validation Checklist

Before releasing v0.9.0:

### Code Quality
- [ ] `cargo build --features lsp` succeeds
- [ ] `cargo test --features lsp` all pass
- [ ] `cargo clippy --features lsp` zero warnings
- [ ] `cargo doc --features lsp` generates without warnings

### Functionality
- [ ] mkdlint-lsp binary runs without errors
- [ ] Initialize handshake completes
- [ ] Diagnostics appear on file open
- [ ] Diagnostics update on file change (debounced)
- [ ] Code actions available for fixable errors
- [ ] Code actions apply correctly
- [ ] "Fix All" command works
- [ ] Config discovery works (finds .markdownlint.json)
- [ ] Shutdown gracefully

### Editor Integration
- [ ] VS Code: diagnostics, quick fixes, fix all
- [ ] Neovim: diagnostics, quick fixes
- [ ] Emacs: diagnostics (basic verification)

### Documentation
- [ ] USER_GUIDE.md LSP section complete
- [ ] LSP.md architecture doc exists
- [ ] README mentions LSP prominently
- [ ] CHANGELOG updated for v0.9.0
- [ ] Editor setup examples tested

### Performance
- [ ] Lints 1,000 line file in < 100ms
- [ ] Debouncing prevents excessive re-lints
- [ ] No memory leaks on long-running sessions
- [ ] Config caching works (doesn't re-read every time)

---

## 10. Conclusion

The LSP implementation is **90% complete**! The hard work is done:
- ✅ Full backend with all handlers
- ✅ Document management
- ✅ Diagnostics conversion
- ✅ Code actions
- ✅ Debouncing
- ✅ Binary entry point

**What's left is polish:**
- ⏳ Config discovery (2-3h)
- ⏳ Comprehensive testing (3-4h)
- ⏳ Documentation (2-3h)

This is achievable in **1-2 days** of focused work. The result will be a production-ready LSP server that makes mkdlint accessible to millions of editor users!

🚀 Let's ship v0.9.0!
