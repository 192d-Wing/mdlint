//! LSP backend implementation
//!
//! This module provides the main Language Server implementation.

use tower_lsp::Client;

/// The mkdlint Language Server
pub struct MkdlintLanguageServer {
    client: Client,
}

impl MkdlintLanguageServer {
    /// Create a new language server instance
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}
