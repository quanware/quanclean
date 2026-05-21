use humansize::{format_size, BINARY};
use quanclean_core::types::{CleanResult, ScanResult};

pub fn format_bytes(bytes: u64) -> String {
    format_size(bytes, BINARY)
}

pub fn print_scan_result(result: &ScanResult, json: bool) {
    if json {
        println!("{}", serde_json::to_string_pretty(result).unwrap());
        return;
    }

    use quanclean_core::types::ScanCategory;
    let categories = [
        ScanCategory::Temp,
        ScanCategory::Cache,
        ScanCategory::Logs,
        ScanCategory::Duplicates,
        ScanCategory::Residue,
    ];

    println!("\nScan Results");
    println!("{}", "─".repeat(50));

    for cat in &categories {
        let entries = result.by_category(cat);
        if entries.is_empty() {
            continue;
        }
        let size: u64 = entries.iter().map(|e| e.size).sum();
        println!("  {:.<35} {:>10}", format!("{cat} "), format_bytes(size));
        println!("  {} file(s)", entries.len());
    }

    println!("{}", "─".repeat(50));
    println!(
        "  Total: {} files   {}",
        result.entries.len(),
        format_bytes(result.total_size)
    );
}

pub fn print_clean_result(result: &CleanResult, json: bool) {
    if json {
        println!("{}", serde_json::to_string_pretty(result).unwrap());
        return;
    }

    println!("\nCleanup Complete");
    println!("{}", "─".repeat(50));
    println!("  Files deleted: {}", result.files_deleted);
    println!("  Space freed:   {}", format_bytes(result.bytes_freed));

    if !result.errors.is_empty() {
        println!("\n  Errors ({}): ", result.errors.len());
        for e in &result.errors {
            println!("    - {e}");
        }
    }
}
