mod commands;
mod output;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "quanclean",
    about = "QuanClean — clean smarter, not harder",
    version,
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Output as JSON
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan disk for cleanable files
    Scan(commands::scan::ScanArgs),
    /// Clean files found by a previous scan
    Clean(commands::clean::CleanArgs),
    /// Find duplicate files
    Duplicates(commands::duplicates::DuplicatesArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan(args) => commands::scan::run(args, cli.json),
        Commands::Clean(args) => commands::clean::run(args, cli.json),
        Commands::Duplicates(args) => commands::duplicates::run(args, cli.json),
    }
}
