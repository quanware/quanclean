use anyhow::Result;
use clap::Args;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use quanclean_core::cleaner;
use quanclean_core::duplicates;
use std::io::{self, Write as IoWrite};
use std::path::PathBuf;
use std::time::Duration;

use crate::output::{format_bytes, print_clean_result, print_scan_result};

#[derive(Args)]
pub struct DuplicatesArgs {
    /// Directories to search for duplicates
    #[arg(required = true, value_name = "DIR")]
    pub paths: Vec<PathBuf>,
    /// Delete duplicates without prompting
    #[arg(long, short = 'y')]
    pub delete: bool,
    /// Dry run — show duplicates without deleting
    #[arg(long)]
    pub dry_run: bool,
}

pub fn run(args: DuplicatesArgs, json: bool) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Hashing files...");

    let dirs: Vec<&std::path::Path> = args.paths.iter().map(|p| p.as_path()).collect();
    let result = duplicates::find_duplicates(&dirs)?;

    pb.finish_and_clear();

    if result.entries.is_empty() {
        if json {
            println!("{{\"duplicates\":[]}}");
        } else {
            println!("No duplicates found.");
        }
        return Ok(());
    }

    if !json {
        println!(
            "Found {} duplicate files using {}.",
            result.entries.len(),
            format_bytes(result.total_size)
        );
    }

    print_scan_result(&result, json);

    if args.dry_run || json {
        return Ok(());
    }

    if !args.delete {
        print!(
            "Delete {} duplicate files? [y/N] ",
            result.entries.len()
        );
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
            println!("Aborted.");
            return Ok(());
        }
    }

    let clean_result = cleaner::clean_files(&result.entries)?;
    print_clean_result(&clean_result, false);

    Ok(())
}
