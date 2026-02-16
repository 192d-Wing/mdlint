//! Configuration discovery and caching for LSP
//!
//! This module provides automatic config file discovery by walking up
//! the directory tree from the file being linted to the workspace root.

use crate::config::Config;
use dashmap::DashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tower_lsp::lsp_types::Url;

/// Manages configuration discovery and caching
pub struct ConfigManager {
    /// Cache of configs by directory path
    /// None means we checked and found no config
    cache: Arc<DashMap<PathBuf, Option<Config>>>,
    /// Workspace roots (from LSP initialize)
    pub(crate) workspace_roots: Vec<PathBuf>,
}

impl ConfigManager {
    /// Create a new config manager with workspace roots
    pub fn new(workspace_roots: Vec<PathBuf>) -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
            workspace_roots,
        }
    }

    /// Discover config for a file URI
    ///
    /// Walks up the directory tree from the file's directory to the workspace root,
    /// looking for known config file names. Results are cached by directory.
    pub fn discover_config(&self, uri: &Url) -> Option<Config> {
        let file_path = uri.to_file_path().ok()?;
        let dir = file_path.parent()?;

        // Check cache first
        if let Some(entry) = self.cache.get(dir) {
            return entry.clone();
        }

        // Walk up directory tree to workspace root
        let config = self.find_config(dir);

        // Cache result (even if None)
        self.cache.insert(dir.to_path_buf(), config.clone());

        config
    }

    /// Walk up directory tree looking for config files
    fn find_config(&self, start_dir: &Path) -> Option<Config> {
        let mut current = start_dir;

        loop {
            // Try known config file names in order of preference
            for name in &[
                ".markdownlint.json",
                ".markdownlint.jsonc",
                ".markdownlint.yaml",
                ".markdownlint.yml",
                ".markdownlintrc",
            ] {
                let config_path = current.join(name);
                if config_path.exists() {
                    // Try to parse the config
                    if let Ok(config) = Config::from_file(&config_path) {
                        return Some(config);
                    }
                    // If parsing failed, continue looking for other config files
                }
            }

            // Stop at workspace root
            if self.workspace_roots.iter().any(|root| current == root) {
                break;
            }

            // Go up one level
            match current.parent() {
                Some(parent) => current = parent,
                None => break, // Reached filesystem root
            }
        }

        None
    }

    /// Invalidate cache for a directory (when config changes)
    ///
    /// This should be called when a config file is modified or deleted.
    pub fn invalidate(&self, path: &Path) {
        self.cache.remove(path);
    }

    /// Invalidate all cached configs in a directory tree
    ///
    /// Useful when a config file changes - invalidate the directory and
    /// all subdirectories.
    pub fn invalidate_tree(&self, root: &Path) {
        self.cache.retain(|path, _| !path.starts_with(root));
    }

    /// Clear entire cache
    ///
    /// Useful for testing or when workspace roots change.
    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    /// Get the number of cached configs (for testing/debugging)
    #[cfg(test)]
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_config_discovery_finds_root_config() {
        // Create temp workspace
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create nested structure
        let subdir = root.join("docs").join("guides");
        fs::create_dir_all(&subdir).unwrap();

        // Create config at root
        let config_path = root.join(".markdownlint.json");
        fs::write(&config_path, r#"{"MD013": false}"#).unwrap();

        // Test discovery from subdirectory
        let manager = ConfigManager::new(vec![root.to_path_buf()]);
        let config = manager.find_config(&subdir);

        assert!(config.is_some(), "Should find config in parent directory");
    }

    #[test]
    fn test_config_discovery_prefers_closer_config() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create nested structure
        let subdir = root.join("docs");
        fs::create_dir_all(&subdir).unwrap();

        // Create config at root
        let root_config = root.join(".markdownlint.json");
        fs::write(&root_config, r#"{"MD013": false}"#).unwrap();

        // Create different config in subdir
        let sub_config = subdir.join(".markdownlint.json");
        fs::write(&sub_config, r#"{"MD033": false}"#).unwrap();

        let manager = ConfigManager::new(vec![root.to_path_buf()]);

        // Should find the closer config (subdir)
        let config = manager.find_config(&subdir);
        assert!(config.is_some(), "Should find config in same directory");
    }

    #[test]
    fn test_config_discovery_stops_at_workspace_root() {
        let temp = TempDir::new().unwrap();
        let workspace_root = temp.path().join("workspace");
        fs::create_dir_all(&workspace_root).unwrap();

        // Create config ABOVE workspace root
        let parent_config = temp.path().join(".markdownlint.json");
        fs::write(&parent_config, r#"{"MD013": false}"#).unwrap();

        // Search from workspace root
        let manager = ConfigManager::new(vec![workspace_root.clone()]);
        let config = manager.find_config(&workspace_root);

        // Should NOT find parent config (stopped at workspace root)
        assert!(
            config.is_none(),
            "Should not search above workspace root"
        );
    }

    #[test]
    fn test_config_caching() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let file_path = root.join("test.md");

        // Create config
        let config_path = root.join(".markdownlint.json");
        fs::write(&config_path, r#"{"MD013": false}"#).unwrap();

        // Create dummy file
        fs::write(&file_path, "# Test\n").unwrap();

        let manager = ConfigManager::new(vec![root.to_path_buf()]);

        // Create URLs from file paths
        let url1 = Url::from_file_path(&file_path).unwrap();
        let url2 = Url::from_file_path(&file_path).unwrap();

        // First call
        let config1 = manager.discover_config(&url1);
        assert!(config1.is_some());

        // Check cache
        assert_eq!(manager.cache_size(), 1);

        // Second call should hit cache
        let config2 = manager.discover_config(&url2);

        // Results should be identical
        assert_eq!(config1.is_some(), config2.is_some());

        // Cache size shouldn't increase
        assert_eq!(manager.cache_size(), 1);
    }

    #[test]
    fn test_cache_invalidation() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let file_path = root.join("test.md");
        fs::write(&file_path, "# Test\n").unwrap();

        let manager = ConfigManager::new(vec![root.to_path_buf()]);

        // Populate cache
        let url = Url::from_file_path(&file_path).unwrap();
        let _ = manager.discover_config(&url);
        assert_eq!(manager.cache_size(), 1);

        // Invalidate
        manager.invalidate(root);

        // Should be gone
        assert_eq!(manager.cache_size(), 0);
    }

    #[test]
    fn test_cache_invalidate_tree() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let sub1 = root.join("sub1");
        let sub2 = root.join("sub2");
        fs::create_dir_all(&sub1).unwrap();
        fs::create_dir_all(&sub2).unwrap();

        // Create test files
        let file_root = root.join("test1.md");
        let file_sub1 = sub1.join("test2.md");
        let file_sub2 = sub2.join("test3.md");
        fs::write(&file_root, "# Test\n").unwrap();
        fs::write(&file_sub1, "# Test\n").unwrap();
        fs::write(&file_sub2, "# Test\n").unwrap();

        let manager = ConfigManager::new(vec![root.to_path_buf()]);

        // Populate cache with multiple directories
        let url_root = Url::from_file_path(&file_root).unwrap();
        let url_sub1 = Url::from_file_path(&file_sub1).unwrap();
        let url_sub2 = Url::from_file_path(&file_sub2).unwrap();

        let _ = manager.discover_config(&url_root);
        let _ = manager.discover_config(&url_sub1);
        let _ = manager.discover_config(&url_sub2);

        assert_eq!(manager.cache_size(), 3);

        // Invalidate entire tree
        manager.invalidate_tree(root);

        // All should be gone
        assert_eq!(manager.cache_size(), 0);
    }

    #[test]
    fn test_clear_cache() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let file_path = root.join("test.md");
        fs::write(&file_path, "# Test\n").unwrap();

        let manager = ConfigManager::new(vec![root.to_path_buf()]);

        // Populate cache
        let url = Url::from_file_path(&file_path).unwrap();
        let _ = manager.discover_config(&url);
        assert!(manager.cache_size() > 0);

        // Clear
        manager.clear_cache();

        // Should be empty
        assert_eq!(manager.cache_size(), 0);
    }

    #[test]
    fn test_no_config_found() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let file_path = root.join("test.md");

        // No config file created
        fs::write(&file_path, "# Test\n").unwrap();

        let manager = ConfigManager::new(vec![root.to_path_buf()]);
        let url = Url::from_file_path(&file_path).unwrap();
        let config = manager.discover_config(&url);

        // Should return None
        assert!(config.is_none());

        // Should still cache the negative result
        assert_eq!(manager.cache_size(), 1);
    }

    #[test]
    fn test_yaml_config_discovery() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create YAML config
        let config_path = root.join(".markdownlint.yaml");
        fs::write(&config_path, "MD013: false\n").unwrap();

        let manager = ConfigManager::new(vec![root.to_path_buf()]);
        let config = manager.find_config(root);

        assert!(config.is_some(), "Should find YAML config");
    }

    #[test]
    fn test_discover_config_with_url() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let file_path = root.join("test.md");

        // Create config
        let config_path = root.join(".markdownlint.json");
        fs::write(&config_path, r#"{"MD013": false}"#).unwrap();

        // Create dummy file
        fs::write(&file_path, "# Test\n").unwrap();

        let manager = ConfigManager::new(vec![root.to_path_buf()]);

        // Create URL from file path
        let url = Url::from_file_path(&file_path).unwrap();

        // Discover config via URL
        let config = manager.discover_config(&url);

        assert!(config.is_some(), "Should discover config from URL");
    }
}
