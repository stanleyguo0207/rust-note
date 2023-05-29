#![allow(unused)]

use std::fmt::format;

use anyhow::{Context, Result};
use clap::Parser;
use log::info;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn find_matches(content: &str, pattern: &str, line_matches: &mut Vec<String>) {
    for line in content.lines() {
        if line.contains(pattern) {
            line_matches.push(line.to_string())
        }
    }
}

#[test]
fn find_a_match() {
    let mut result: Vec<String> = Vec::new();
    find_matches(
        "lorem ipsum\ndolor sit amet\nlorem 222",
        "lorem",
        &mut result,
    );
    assert_eq!(result.len(), 2);
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("count not read file `{}`", args.path.display()))?;

    let mut lines: Vec<String> = Vec::new();
    find_matches(&content, &args.pattern, &mut lines);
    for line in lines.iter() {
        info!("{}", line);
    }

    Ok(())
}
