use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScanCategory {
    Temp,
    Cache,
    Logs,
    Duplicates,
    Residue,
}

impl std::fmt::Display for ScanCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanCategory::Temp => write!(f, "Temporary Files"),
            ScanCategory::Cache => write!(f, "Application Cache"),
            ScanCategory::Logs => write!(f, "Log Files"),
            ScanCategory::Duplicates => write!(f, "Duplicate Files"),
            ScanCategory::Residue => write!(f, "App Residue"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub category: ScanCategory,
    /// Optional: hash for duplicate detection
    pub hash: Option<String>,
}

impl FileEntry {
    pub fn new(path: PathBuf, size: u64, category: ScanCategory) -> Self {
        Self { path, size, category, hash: None }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ScanResult {
    pub entries: Vec<FileEntry>,
    pub total_size: u64,
}

impl ScanResult {
    pub fn add(&mut self, entry: FileEntry) {
        self.total_size += entry.size;
        self.entries.push(entry);
    }

    pub fn merge(&mut self, other: ScanResult) {
        self.total_size += other.total_size;
        self.entries.extend(other.entries);
    }

    pub fn by_category(&self, category: &ScanCategory) -> Vec<&FileEntry> {
        self.entries.iter().filter(|e| &e.category == category).collect()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CleanResult {
    pub files_deleted: usize,
    pub bytes_freed: u64,
    pub errors: Vec<String>,
}
