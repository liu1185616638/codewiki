//! Pure domain models shared by library consumers, the CLI, and future adapters.

use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, CodeWikiError>;

#[derive(Debug, thiserror::Error)]
pub enum CodeWikiError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("repository traversal failed: {0}")]
    Walk(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

/// Evidence gathered from one file. This is *not* a semantic AST symbol.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceFile {
    /// Slash-separated path relative to the scanned root.
    pub path: String,
    pub language: String,
    pub size_bytes: u64,
    pub sha256: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScanStats {
    pub included_files: usize,
    pub skipped_sensitive: usize,
    pub skipped_oversized: usize,
    pub skipped_binary_or_non_utf8: usize,
    pub skipped_unsupported: usize,
}

/// A working-tree snapshot. A verified Git commit snapshot is a later phase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanReport {
    pub root: String,
    pub files: Vec<SourceFile>,
    pub stats: ScanStats,
}

/// A portable, storage-independent Wiki page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WikiPage {
    /// Relative output path, controlled by the generator rather than the input repository.
    pub path: String,
    pub title: String,
    pub markdown: String,
    /// Repository-relative source paths that support this page.
    pub sources: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WikiBundle {
    pub pages: Vec<WikiPage>,
}
