//! CLI adapter. Domain logic lives in reusable libraries.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use codewiki_analysis::scan_repository;
use codewiki_core::Result;
use codewiki_wiki::{compile_outline, write_markdown};

#[derive(Debug, Parser)]
#[command(name = "codewiki", version, about = "Source-backed code Wiki toolkit")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Scan the working tree without executing repository code or invoking AI.
    Scan {
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Print the full scan report as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Generate a deterministic Markdown inventory (not yet AI documentation).
    Generate {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short, long, default_value = "./codewiki-output")]
        output: PathBuf,
        /// Explicitly replace existing generated pages.
        #[arg(long)]
        overwrite: bool,
    },
}

fn run() -> Result<()> {
    match Cli::parse().command {
        Command::Scan { path, json } => {
            let report = scan_repository(path)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("Included files: {}", report.stats.included_files);
                println!("Sensitive files skipped: {}", report.stats.skipped_sensitive);
                println!("Oversized files skipped: {}", report.stats.skipped_oversized);
                println!(
                    "Binary/non-UTF8 files skipped: {}",
                    report.stats.skipped_binary_or_non_utf8
                );
                println!("Unsupported files skipped: {}", report.stats.skipped_unsupported);
            }
        }
        Command::Generate {
            path,
            output,
            overwrite,
        } => {
            let report = scan_repository(path)?;
            let wiki = compile_outline(&report);
            write_markdown(&wiki, &output, overwrite)?;
            println!("Generated {} pages in {}", wiki.pages.len(), output.display());
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("codewiki: {error}");
            ExitCode::FAILURE
        }
    }
}
