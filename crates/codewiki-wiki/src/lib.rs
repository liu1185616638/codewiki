//! Wiki document compiler. Generated content is a source inventory, not an AI explanation.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path};

use codewiki_core::{CodeWikiError, Result, ScanReport, SourceFile, WikiBundle, WikiPage};
use sha2::{Digest, Sha256};

/// Build a deterministic, source-backed overview and one page per top-level folder.
pub fn compile_outline(report: &ScanReport) -> WikiBundle {
    let mut modules: BTreeMap<String, Vec<&SourceFile>> = BTreeMap::new();
    for file in &report.files {
        let module = file
            .path
            .split_once('/')
            .map(|(prefix, _)| prefix)
            .unwrap_or("(root)");
        modules.entry(module.to_owned()).or_default().push(file);
    }

    let mut overview = String::from(
        "# Repository overview\n\n\
         > This is a deterministic source inventory, not AI-generated architecture analysis.\n\n\
         The paths below refer to the scanned working tree; no Git commit is pinned yet.\n\n",
    );
    overview.push_str(&format!(
        "- Included files: {}\n- Excluded sensitive files: {}\n- Skipped oversized files: {}\n\
         - Skipped binary/non-UTF8 files: {}\n- Skipped unsupported files: {}\n\n\
         ## Modules\n\n",
        report.stats.included_files,
        report.stats.skipped_sensitive,
        report.stats.skipped_oversized,
        report.stats.skipped_binary_or_non_utf8,
        report.stats.skipped_unsupported,
    ));

    let mut pages = Vec::new();
    for (module, files) in modules {
        let digest = format!("{:x}", Sha256::digest(module.as_bytes()));
        let basename: String = module
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                    ch
                } else {
                    '-'
                }
            })
            .collect();
        let basename = if basename.trim_matches('-').is_empty() {
            "module".to_owned()
        } else {
            basename
        };
        let path = format!("modules/{basename}-{}.md", &digest[..10]);
        overview.push_str(&format!(
            "- [{}]({}) ({} files)\n",
            escape_markdown(&module),
            path,
            files.len()
        ));

        let mut markdown = format!(
            "# {}\n\n> Generated from a deterministic file inventory. \
             Symbols, calls and architecture descriptions are not yet analyzed.\n\n\
             ## Source files\n\n",
            escape_markdown(&module)
        );
        for file in &files {
            markdown.push_str(&format!(
                "- {} — {} ({} bytes, sha256: {})\n",
                escape_markdown(&file.path),
                file.language,
                file.size_bytes,
                file.sha256,
            ));
        }
        pages.push(WikiPage {
            path,
            title: module,
            markdown,
            sources: files.iter().map(|file| file.path.clone()).collect(),
        });
    }
    pages.insert(
        0,
        WikiPage {
            path: "overview.md".to_owned(),
            title: "Repository overview".to_owned(),
            markdown: overview,
            sources: report.files.iter().map(|file| file.path.clone()).collect(),
        },
    );
    WikiBundle { pages }
}

/// Refuses to overwrite existing pages unless explicitly requested.
pub fn write_markdown(
    bundle: &WikiBundle,
    output: impl AsRef<Path>,
    overwrite: bool,
) -> Result<()> {
    let output = output.as_ref();
    for page in &bundle.pages {
        if Path::new(&page.path)
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
        {
            return Err(CodeWikiError::InvalidInput(format!(
                "unsafe Wiki page path: {}",
                page.path
            )));
        }
        let destination = output.join(&page.path);
        if destination.exists() && !overwrite {
            return Err(CodeWikiError::InvalidInput(format!(
                "page already exists: {} (pass --overwrite to replace generated files)",
                destination.display()
            )));
        }
    }
    for page in &bundle.pages {
        let destination = output.join(&page.path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(destination, page.markdown.as_bytes())?;
    }
    Ok(())
}

fn escape_markdown(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('(', "\\(")
        .replace(')', "\\)")
        .replace(['\n', '\r'], " ")
}

#[cfg(test)]
mod tests {
    use codewiki_core::{ScanReport, ScanStats, SourceFile, WikiBundle, WikiPage};

    use super::{compile_outline, write_markdown};

    #[test]
    fn builds_and_exports_stable_outline_without_overwrite() {
        let report = ScanReport {
            root: "example".into(),
            files: vec![SourceFile {
                path: "src/main.rs".into(),
                language: "Rust".into(),
                size_bytes: 12,
                sha256: "a".repeat(64),
            }],
            stats: ScanStats {
                included_files: 1,
                ..ScanStats::default()
            },
        };
        let bundle = compile_outline(&report);
        assert_eq!(bundle.pages.len(), 2);
        assert!(bundle.pages[0].markdown.contains("Modules"));
        assert_eq!(bundle.pages[1].sources, vec!["src/main.rs"]);
        let temp = tempfile::tempdir().unwrap();
        write_markdown(&bundle, temp.path(), false).unwrap();
        assert!(temp.path().join("overview.md").exists());
        assert!(write_markdown(&bundle, temp.path(), false).is_err());
        write_markdown(&bundle, temp.path(), true).unwrap();
    }

    #[test]
    fn rejects_path_traversal() {
        let temp = tempfile::tempdir().unwrap();
        let bundle = WikiBundle {
            pages: vec![WikiPage {
                path: "../escape.md".into(),
                title: "bad".into(),
                markdown: "not allowed".into(),
                sources: vec![],
            }],
        };
        assert!(write_markdown(&bundle, temp.path(), false).is_err());
        assert!(!temp.path().join("../escape.md").exists());
    }
}
