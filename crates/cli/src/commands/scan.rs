use anyhow::Result;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use quanclean_core::scanner;
use quanclean_core::types::ScanCategory;
use std::path::PathBuf;
use std::time::Duration;

use crate::output::{format_bytes, print_scan_result};

#[derive(Args)]
pub struct ScanArgs {
    /// Scan temp files only
    #[arg(long)]
    pub temp: bool,
    /// Scan cache files only
    #[arg(long)]
    pub cache: bool,
    /// Scan log files only
    #[arg(long)]
    pub logs: bool,
    /// Custom directories to scan (space-separated)
    #[arg(long, value_name = "DIR")]
    pub path: Vec<PathBuf>,
    /// Minimum file size to report (e.g. 1MB)
    #[arg(long, default_value = "0")]
    pub min_size: u64,
}

pub fn run(args: ScanArgs, json: bool) -> Result<()> {
    if !json {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_message("Scanning...");

        let mut result = if !args.path.is_empty() {
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
            result.entries.retain(|e| e.size >= args.min_size);
            result.total_size = result.entries.iter().map(|e| e.size).sum();
        }

        pb.finish_and_clear();

        if result.entries.is_empty() {
            println!("No cleanable files found.");
        } else {
            println!(
                "Found {} of cleanable files across {} items.",
                format_bytes(result.total_size),
                result.entries.len()
            );
            print_scan_result(&result, false);
        }
    } else {
        let mut result = if !args.path.is_empty() {
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
            result.entries.retain(|e| e.size >= args.min_size);
            result.total_size = result.entries.iter().map(|e| e.size).sum();
        }

        print_scan_result(&result, true);
    }

    Ok(())
}
