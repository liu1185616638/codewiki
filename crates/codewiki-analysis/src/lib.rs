//! Repository discovery. Future versions add Tree-sitter and language-indexer adapters.
//! This initial scanner does not execute code and does not call a language model.

use std::fs;
use std::path::Path;

use codewiki_core::{CodeWikiError, Result, ScanReport, ScanStats, SourceFile};
use ignore::{DirEntry, WalkBuilder};
use sha2::{Digest, Sha256};

pub const MAX_SOURCE_BYTES: u64 = 1024 * 1024;

/// Scan a local directory deterministically, respecting Git ignore rules.
pub fn scan_repository(root: impl AsRef<Path>) -> Result<ScanReport> {
    let root = root.as_ref().canonicalize()?;
    if !root.is_dir() {
        return Err(CodeWikiError::InvalidInput(format!(
            "not a directory: {}",
            root.display()
        )));
    }

    let mut walker = WalkBuilder::new(&root);
    walker
        .hidden(false)
        .git_ignore(true)
        .git_exclude(true)
        .require_git(false)
        .follow_links(false)
        .add_custom_ignore_filename(".codewikiignore")
        .filter_entry(should_descend);

    let mut files = Vec::new();
    let mut stats = ScanStats::default();

    for result in walker.build() {
        let entry = result.map_err(|error| CodeWikiError::Walk(error.to_string()))?;
        if !entry.file_type().is_some_and(|kind| kind.is_file()) {
            continue;
        }

        let path = entry.path();
        if is_sensitive(path) {
            stats.skipped_sensitive += 1;
            continue;
        }

        let Some(language) = classify(path) else {
            stats.skipped_unsupported += 1;
            continue;
        };

        let metadata = fs::metadata(path)?;
        if metadata.len() > MAX_SOURCE_BYTES {
            stats.skipped_oversized += 1;
            continue;
        }

        let bytes = fs::read(path)?;
        if bytes.contains(&0) || std::str::from_utf8(&bytes).is_err() {
            stats.skipped_binary_or_non_utf8 += 1;
            continue;
        }

        let relative = path.strip_prefix(&root).map_err(|error| {
            CodeWikiError::InvalidInput(format!("invalid relative path: {error}"))
        })?;
        let normalized = relative.to_string_lossy().replace('\\', "/");
        files.push(SourceFile {
            path: normalized,
            language: language.to_owned(),
            size_bytes: bytes.len() as u64,
            sha256: format!("{:x}", Sha256::digest(&bytes)),
        });
    }

    files.sort_by(|a, b| a.path.cmp(&b.path));
    stats.included_files = files.len();
    Ok(ScanReport {
        root: root.to_string_lossy().into_owned(),
        files,
        stats,
    })
}

fn should_descend(entry: &DirEntry) -> bool {
    if entry.depth() == 0 {
        return true;
    }
    if !entry.file_type().is_some_and(|kind| kind.is_dir()) {
        return true;
    }
    let name = entry.file_name().to_string_lossy();
    !matches!(
        name.as_ref(),
        ".git" | "node_modules" | "target" | "dist" | "build" | "coverage" | ".venv"
    )
}

fn is_sensitive(path: &Path) -> bool {
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    name == ".env"
        || name.starts_with(".env.")
        || matches!(
            name.as_str(),
            ".npmrc" | ".pypirc" | "id_rsa" | "id_ed25519" | "credentials"
        )
        || matches!(
            path.extension().and_then(|value| value.to_str()),
            Some("pem" | "p12" | "pfx" | "key")
        )
}

fn classify(path: &Path) -> Option<&'static str> {
    let name = path.file_name()?.to_str()?;
    if name == "Dockerfile" {
        return Some("Dockerfile");
    }
    match path.extension()?.to_str()?.to_ascii_lowercase().as_str() {
        "rs" => Some("Rust"),
        "py" => Some("Python"),
        "java" => Some("Java"),
        "kt" | "kts" => Some("Kotlin"),
        "go" => Some("Go"),
        "js" | "jsx" | "mjs" | "cjs" => Some("JavaScript"),
        "ts" | "tsx" => Some("TypeScript"),
        "c" | "h" => Some("C"),
        "cc" | "cpp" | "cxx" | "hpp" => Some("C++"),
        "cs" => Some("C#"),
        "php" => Some("PHP"),
        "rb" => Some("Ruby"),
        "md" | "mdx" => Some("Markdown"),
        "toml" => Some("TOML"),
        "yaml" | "yml" => Some("YAML"),
        "json" => Some("JSON"),
        "xml" => Some("XML"),
        "sh" | "bash" | "ps1" => Some("Shell"),
        "sql" => Some("SQL"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::scan_repository;

    #[test]
    fn respects_gitignore_and_excludes_credentials() {
        let temp = tempfile::tempdir().expect("temporary directory");
        fs::create_dir(temp.path().join("src")).unwrap();
        fs::create_dir(temp.path().join("target")).unwrap();
        fs::write(temp.path().join(".gitignore"), "ignored.rs\n").unwrap();
        fs::write(temp.path().join("ignored.rs"), "pub fn ignored() {}\n").unwrap();
        fs::write(temp.path().join(".env"), "SECRET=hidden\n").unwrap();
        fs::write(temp.path().join("src/main.rs"), "fn main() {}\n").unwrap();
        fs::write(temp.path().join("target/secret.rs"), "fn secret() {}\n").unwrap();

        let report = scan_repository(temp.path()).unwrap();
        assert_eq!(report.stats.included_files, 1);
        assert_eq!(report.files[0].path, "src/main.rs");
        assert_eq!(report.files[0].language, "Rust");
        assert_eq!(report.files[0].sha256.len(), 64);
        assert!(
            report
                .files
                .iter()
                .all(|file| !file.path.contains("secret"))
        );
    }

    #[test]
    fn rejects_file_as_repository_root() {
        let temp = tempfile::tempdir().unwrap();
        let file = temp.path().join("file.rs");
        fs::write(&file, "fn main() {}").unwrap();
        assert!(scan_repository(&file).is_err());
    }
}
