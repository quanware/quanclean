use anyhow::Result;
use clap::Args;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use quanclean_core::cleaner;
use quanclean_core::scanner;
use quanclean_core::types::ScanCategory;
use std::io::{self, Write as IoWrite};
use std::path::PathBuf;
use std::time::Duration;

use crate::output::{format_bytes, print_clean_result};

#[derive(Args)]
pub struct CleanArgs {
    /// Skip confirmation prompt
    #[arg(long, short = 'y')]
    pub yes: bool,
    /// Clean temp files only
    #[arg(long)]
    pub temp: bool,
    /// Clean cache files only
    #[arg(long)]
    pub cache: bool,
    /// Clean log files only
    #[arg(long)]
    pub logs: bool,
    /// Custom directories to clean (space-separated)
    #[arg(long, value_name = "DIR")]
    pub path: Vec<PathBuf>,
    /// Minimum file size to include (bytes)
    #[arg(long, default_value = "0")]
    pub min_size: u64,
    /// Dry run — show what would be deleted without deleting
    #[arg(long)]
    pub dry_run: bool,
}

pub fn run(args: CleanArgs, json: bool) -> Result<()> {
    // Scan first
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Scanning...");

    let mut scan_result = if !args.path.is_empty() {
        let mut combined = quanclean_core::types::ScanResult::default();
        for p in &args.path {
            let partial = scanner::scan_directory(p, ScanCategory::Temp)?;
            combined.merge(partial);
        }
        combined
    } else if args.temp {
        scanner::scan_temp()?
    } else if args.cache {
        scanner::scan_cache()?
    } else if args.logs {
        scanner::scan_logs()?
    } else {
        scanner::full_scan()?
    };

    if args.min_size > 0 {
        scan_result.entries.retain(|e| e.size >= args.min_size);
        scan_result.total_size = scan_result.entries.iter().map(|e| e.size).sum();
    }

    pb.finish_and_clear();

    if scan_result.entries.is_empty() {
        println!("Nothing to clean.");
        return Ok(());
    }

    if !json {
        println!(
            "Found {} ({} files) to clean.",
            format_bytes(scan_result.total_size),
            scan_result.entries.len()
        );
    }

    if args.dry_run {
        if json {
            println!("{}", serde_json::to_string_pretty(&scan_result)?);
        } else {
            println!("{}", style("[dry-run] No files were deleted.").yellow());
            for e in &scan_result.entries {
                println!("  would delete: {}", e.path.display());
            }
        }
        return Ok(());
    }

    // Confirm
    if !args.yes && !json {
        print!(
            "Are you sure you want to delete {} of files? [y/N] ",
            format_bytes(scan_result.total_size)
        );
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
            println!("Aborted.");
            return Ok(());
        }
    }

    // Clean
    let clean_pb = ProgressBar::new(scan_result.entries.len() as u64);
    clean_pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.cyan} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );
    clean_pb.set_message("Cleaning...");

    let result = cleaner::clean_files(&scan_result.entries)?;
    clean_pb.finish_and_clear();

    print_clean_result(&result, json);

    Ok(())
}
