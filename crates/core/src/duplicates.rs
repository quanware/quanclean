use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;

use crate::error::Result;
use crate::types::{FileEntry, ScanCategory, ScanResult};

const HASH_BUF_SIZE: usize = 64 * 1024; // 64 KB read buffer

/// Hash a file using SHA-256, returning the hex string.
pub fn hash_file(path: &Path) -> Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; HASH_BUF_SIZE];

    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }

    Ok(hex::encode(hasher.finalize()))
}

/// Scan a list of directories for duplicate files.
/// Returns a ScanResult containing all duplicate copies (keeping the first occurrence of each hash).
pub fn find_duplicates(dirs: &[&Path]) -> Result<ScanResult> {
    let mut size_groups: HashMap<u64, Vec<FileEntry>> = HashMap::new();

    // First pass: group by file size (cheap filter)
    for &dir in dirs {
        if !dir.exists() {
            continue;
        }
        for entry in walkdir::WalkDir::new(dir)
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
            // Skip empty files — they're all "duplicates" but useless to report
            if size == 0 {
                continue;
            }
            size_groups
                .entry(size)
                .or_default()
                .push(FileEntry::new(path, size, ScanCategory::Duplicates));
        }
    }

    // Second pass: hash files that share a size
    let mut hash_groups: HashMap<String, Vec<FileEntry>> = HashMap::new();

    for (_size, candidates) in size_groups.into_iter().filter(|(_, v)| v.len() > 1) {
        for mut entry in candidates {
            match hash_file(&entry.path) {
                Ok(hash) => {
                    entry.hash = Some(hash.clone());
                    hash_groups.entry(hash).or_default().push(entry);
                }
                Err(_) => continue, // skip unreadable files
            }
        }
    }

    let mut result = ScanResult::default();

    // Emit all copies except the first (the "original" we keep)
    for (_hash, mut group) in hash_groups.into_iter().filter(|(_, v)| v.len() > 1) {
        group.sort_by(|a, b| a.path.cmp(&b.path)); // deterministic order
        for duplicate in group.into_iter().skip(1) {
            result.add(duplicate);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn hash_file_is_deterministic() {
        let tmp = TempDir::new().unwrap();
        let f = tmp.path().join("test.bin");
        fs::write(&f, b"deterministic content").unwrap();

        let h1 = hash_file(&f).unwrap();
        let h2 = hash_file(&f).unwrap();
        assert_eq!(h1, h2);
    }

    #[test]
    fn hash_file_differs_for_different_content() {
        let tmp = TempDir::new().unwrap();
        let f1 = tmp.path().join("a.bin");
        let f2 = tmp.path().join("b.bin");
        fs::write(&f1, b"content A").unwrap();
        fs::write(&f2, b"content B").unwrap();

        assert_ne!(hash_file(&f1).unwrap(), hash_file(&f2).unwrap());
    }

    #[test]
    fn find_duplicates_detects_copies() {
        let tmp = TempDir::new().unwrap();
        let original = tmp.path().join("original.txt");
        let copy1 = tmp.path().join("copy1.txt");
        let copy2 = tmp.path().join("copy2.txt");
        let unique = tmp.path().join("unique.txt");

        let content = b"same content in all three";
        fs::write(&original, content).unwrap();
        fs::write(&copy1, content).unwrap();
        fs::write(&copy2, content).unwrap();
        fs::write(&unique, b"different content here").unwrap();

        let result = find_duplicates(&[tmp.path()]).unwrap();

        // 3 copies → 2 duplicates (skip first/original)
        assert_eq!(result.entries.len(), 2);
        assert_eq!(result.total_size, 2 * content.len() as u64);
    }

    #[test]
    fn find_duplicates_ignores_empty_files() {
        let tmp = TempDir::new().unwrap();
        for name in &["a.txt", "b.txt", "c.txt"] {
            fs::write(tmp.path().join(name), b"").unwrap();
        }
        let result = find_duplicates(&[tmp.path()]).unwrap();
        assert!(result.entries.is_empty());
    }

    #[test]
    fn find_duplicates_no_duplicates_returns_empty() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("a.txt"), b"aaa").unwrap();
        fs::write(tmp.path().join("b.txt"), b"bbb").unwrap();

        let result = find_duplicates(&[tmp.path()]).unwrap();
        assert!(result.entries.is_empty());
    }
}
