use std::fs;

use crate::error::{CoreError, Result};
use crate::types::{CleanResult, FileEntry};

/// Delete a single file, updating the result counters.
fn delete_file(entry: &FileEntry, result: &mut CleanResult) {
    match fs::remove_file(&entry.path) {
        Ok(()) => {
            result.files_deleted += 1;
            result.bytes_freed += entry.size;
        }
        Err(e) => {
            result.errors.push(format!("{}: {}", entry.path.display(), e));
        }
    }
}

/// Clean all files in the provided entries list.
pub fn clean_files(entries: &[FileEntry]) -> Result<CleanResult> {
    let mut result = CleanResult::default();

    for entry in entries {
        if !entry.path.exists() {
            continue;
        }
        delete_file(entry, &mut result);
    }

    Ok(result)
}

/// Clean files that match a specific predicate (e.g. only Temp category).
pub fn clean_filtered<F>(entries: &[FileEntry], predicate: F) -> Result<CleanResult>
where
    F: Fn(&FileEntry) -> bool,
{
    let filtered: Vec<&FileEntry> = entries.iter().filter(|e| predicate(e)).collect();
    let mut result = CleanResult::default();
    for entry in filtered {
        if !entry.path.exists() {
            continue;
        }
        delete_file(entry, &mut result);
    }
    Ok(result)
}

/// Remove empty directories recursively from the given root.
pub fn remove_empty_dirs(root: &std::path::Path) -> Result<usize> {
    let mut count = 0;

    if !root.is_dir() {
        return Ok(0);
    }

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            count += remove_empty_dirs(&path)?;
        }
    }

    // Try to remove this directory if now empty
    if fs::read_dir(root)?.next().is_none() {
        match fs::remove_dir(root) {
            Ok(()) => count += 1,
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {}
            Err(e) => return Err(CoreError::Io(e)),
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ScanCategory;
    use std::fs;
    use tempfile::TempDir;

    fn make_entry(dir: &std::path::Path, name: &str, content: &[u8]) -> FileEntry {
        let path = dir.join(name);
        fs::write(&path, content).unwrap();
        FileEntry::new(path, content.len() as u64, ScanCategory::Temp)
    }

    #[test]
    fn clean_files_deletes_and_counts() {
        let tmp = TempDir::new().unwrap();
        let e1 = make_entry(tmp.path(), "a.tmp", b"hello");
        let e2 = make_entry(tmp.path(), "b.tmp", b"world!");

        let result = clean_files(&[e1, e2]).unwrap();
        assert_eq!(result.files_deleted, 2);
        assert_eq!(result.bytes_freed, 11);
        assert!(result.errors.is_empty());
        assert!(!tmp.path().join("a.tmp").exists());
        assert!(!tmp.path().join("b.tmp").exists());
    }

    #[test]
    fn clean_files_skips_already_deleted() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("ghost.tmp");
        let entry = FileEntry::new(path, 100, ScanCategory::Temp);
        // file was never created — should skip silently
        let result = clean_files(&[entry]).unwrap();
        assert_eq!(result.files_deleted, 0);
        assert_eq!(result.bytes_freed, 0);
    }

    #[test]
    fn clean_filtered_only_deletes_matching() {
        let tmp = TempDir::new().unwrap();
        let e_temp = make_entry(tmp.path(), "a.tmp", b"temp");
        let cache_path = tmp.path().join("b.cache");
        fs::write(&cache_path, b"cache data").unwrap();
        let e_cache = FileEntry::new(cache_path, 10, ScanCategory::Cache);

        let all = vec![e_temp, e_cache];
        let result = clean_filtered(&all, |e| e.category == ScanCategory::Temp).unwrap();

        assert_eq!(result.files_deleted, 1);
        assert!(!tmp.path().join("a.tmp").exists());
        assert!(tmp.path().join("b.cache").exists());
    }

    #[test]
    fn remove_empty_dirs_removes_empty_subtree() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("empty_sub");
        fs::create_dir_all(&sub).unwrap();

        let count = remove_empty_dirs(tmp.path()).unwrap();
        assert!(count >= 1);
        assert!(!sub.exists());
    }
}
