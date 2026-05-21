use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::error::Result;
use crate::types::{FileEntry, ScanCategory, ScanResult};

/// Collect all scannable directories for the current platform.
pub fn default_scan_paths() -> Vec<(PathBuf, ScanCategory)> {
    #[cfg(target_os = "windows")]
    return windows_scan_paths();

    #[cfg(target_os = "macos")]
    return macos_scan_paths();

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    return vec![];
}

#[cfg(target_os = "windows")]
fn windows_scan_paths() -> Vec<(PathBuf, ScanCategory)> {
    let mut paths = Vec::new();

    // %TEMP% and %TMP%
    for var in &["TEMP", "TMP"] {
        if let Ok(p) = std::env::var(var) {
            paths.push((PathBuf::from(p), ScanCategory::Temp));
        }
    }

    // Windows system temp
    if let Ok(windir) = std::env::var("SystemRoot") {
        paths.push((PathBuf::from(&windir).join("Temp"), ScanCategory::Temp));
        paths.push((PathBuf::from(&windir).join("SoftwareDistribution").join("Download"), ScanCategory::Cache));
    }

    // User profile based paths
    if let Ok(userprofile) = std::env::var("USERPROFILE") {
        let base = PathBuf::from(&userprofile);

        // AppData\Local\Temp
        paths.push((base.join("AppData").join("Local").join("Temp"), ScanCategory::Temp));

        // Application caches
        let local_app_data = base.join("AppData").join("Local");
        for cache_subdir in &[
            "Microsoft\\Windows\\INetCache",
            "Microsoft\\Windows\\Temporary Internet Files",
            "Google\\Chrome\\User Data\\Default\\Cache",
            "Microsoft\\Edge\\User Data\\Default\\Cache",
            "Mozilla\\Firefox\\Profiles",
        ] {
            let p = local_app_data.join(cache_subdir);
            paths.push((p, ScanCategory::Cache));
        }

        // Prefetch
        if let Ok(windir) = std::env::var("SystemRoot") {
            paths.push((PathBuf::from(windir).join("Prefetch"), ScanCategory::Cache));
        }

        // Log files
        paths.push((base.join("AppData").join("Local").join("Microsoft").join("Windows").join("WER"), ScanCategory::Logs));
    }

    paths
}

#[cfg(target_os = "macos")]
fn macos_scan_paths() -> Vec<(PathBuf, ScanCategory)> {
    let mut paths = Vec::new();

    // /tmp and /var/folders (macOS temp)
    paths.push((PathBuf::from("/tmp"), ScanCategory::Temp));
    paths.push((PathBuf::from("/private/tmp"), ScanCategory::Temp));
    paths.push((PathBuf::from("/var/folders"), ScanCategory::Temp));

    if let Ok(home) = std::env::var("HOME") {
        let base = PathBuf::from(&home);

        // ~/Library/Caches
        paths.push((base.join("Library").join("Caches"), ScanCategory::Cache));

        // ~/Library/Logs
        paths.push((base.join("Library").join("Logs"), ScanCategory::Logs));

        // ~/Library/Application Support/.../Cache
        paths.push((base.join("Library").join("Application Support"), ScanCategory::Cache));
    }

    // System caches (requires permissions on modern macOS)
    paths.push((PathBuf::from("/Library/Caches"), ScanCategory::Cache));
    paths.push((PathBuf::from("/System/Library/Caches"), ScanCategory::Cache));

    paths
}

/// Scan a single directory for files, returning entries with size and category.
pub fn scan_directory(dir: &Path, category: ScanCategory) -> Result<ScanResult> {
    let mut result = ScanResult::default();

    if !dir.exists() {
        return Ok(result);
    }

    for entry in WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path().to_path_buf();
        if !path.is_file() {
            continue;
        }

        let size = match entry.metadata() {
            Ok(m) => m.len(),
            Err(_) => continue,
        };

        result.add(FileEntry::new(path, size, category.clone()));
    }

    Ok(result)
}

/// Run a full scan across all default paths for this platform.
pub fn full_scan() -> Result<ScanResult> {
    let mut combined = ScanResult::default();

    for (path, category) in default_scan_paths() {
        let partial = scan_directory(&path, category)?;
        combined.merge(partial);
    }

    Ok(combined)
}

/// Scan only temp file locations.
pub fn scan_temp() -> Result<ScanResult> {
    let mut combined = ScanResult::default();

    for (path, category) in default_scan_paths() {
        if category == ScanCategory::Temp {
            let partial = scan_directory(&path, category)?;
            combined.merge(partial);
        }
    }

    Ok(combined)
}

/// Scan only cache locations.
pub fn scan_cache() -> Result<ScanResult> {
    let mut combined = ScanResult::default();

    for (path, category) in default_scan_paths() {
        if category == ScanCategory::Cache {
            let partial = scan_directory(&path, category)?;
            combined.merge(partial);
        }
    }

    Ok(combined)
}

/// Scan only log file locations.
pub fn scan_logs() -> Result<ScanResult> {
    let mut combined = ScanResult::default();

    for (path, category) in default_scan_paths() {
        if category == ScanCategory::Logs {
            let partial = scan_directory(&path, category)?;
            combined.merge(partial);
        }
    }

    Ok(combined)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_temp_tree(dir: &Path) -> Vec<PathBuf> {
        let files = vec![
            dir.join("a.tmp"),
            dir.join("b.tmp"),
            dir.join("sub").join("c.tmp"),
        ];
        fs::create_dir_all(dir.join("sub")).unwrap();
        for f in &files {
            fs::write(f, b"hello").unwrap();
        }
        files
    }

    #[test]
    fn scan_directory_counts_files() {
        let tmp = TempDir::new().unwrap();
        let files = make_temp_tree(tmp.path());
        let result = scan_directory(tmp.path(), ScanCategory::Temp).unwrap();
        assert_eq!(result.entries.len(), files.len());
    }

    #[test]
    fn scan_directory_totals_size() {
        let tmp = TempDir::new().unwrap();
        make_temp_tree(tmp.path());
        let result = scan_directory(tmp.path(), ScanCategory::Temp).unwrap();
        // each file has 5 bytes ("hello"), 3 files
        assert_eq!(result.total_size, 15);
    }

    #[test]
    fn scan_directory_nonexistent_returns_empty() {
        let result = scan_directory(Path::new("/nonexistent/path/xyz"), ScanCategory::Temp).unwrap();
        assert!(result.entries.is_empty());
        assert_eq!(result.total_size, 0);
    }

    #[test]
    fn scan_result_by_category() {
        let tmp = TempDir::new().unwrap();
        make_temp_tree(tmp.path());
        let mut result = scan_directory(tmp.path(), ScanCategory::Temp).unwrap();

        // Add one cache entry manually
        let cache_path = tmp.path().join("cache.bin");
        fs::write(&cache_path, b"cache").unwrap();
        result.add(FileEntry::new(cache_path, 5, ScanCategory::Cache));

        let temp_entries = result.by_category(&ScanCategory::Temp);
        let cache_entries = result.by_category(&ScanCategory::Cache);
        assert_eq!(temp_entries.len(), 3);
        assert_eq!(cache_entries.len(), 1);
    }

    #[test]
    fn scan_result_merge() {
        let tmp1 = TempDir::new().unwrap();
        let tmp2 = TempDir::new().unwrap();
        fs::write(tmp1.path().join("a.tmp"), b"aaaaa").unwrap();
        fs::write(tmp2.path().join("b.tmp"), b"bb").unwrap();

        let r1 = scan_directory(tmp1.path(), ScanCategory::Temp).unwrap();
        let r2 = scan_directory(tmp2.path(), ScanCategory::Cache).unwrap();
        let mut combined = ScanResult::default();
        combined.merge(r1);
        combined.merge(r2);

        assert_eq!(combined.entries.len(), 2);
        assert_eq!(combined.total_size, 7);
    }
}
