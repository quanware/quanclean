use quanclean_core::cleaner;
use quanclean_core::duplicates;
use quanclean_core::scanner;
use quanclean_core::types::{CleanResult, FileEntry, ScanResult};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct CommandError {
    pub message: String,
}

impl From<quanclean_core::CoreError> for CommandError {
    fn from(e: quanclean_core::CoreError) -> Self {
        CommandError { message: e.to_string() }
    }
}

type CmdResult<T> = Result<T, CommandError>;

#[tauri::command]
pub async fn scan_full() -> CmdResult<ScanResult> {
    tokio::task::spawn_blocking(scanner::full_scan)
        .await
        .map_err(|e| CommandError { message: e.to_string() })?
        .map_err(Into::into)
}

#[tauri::command]
pub async fn scan_temp() -> CmdResult<ScanResult> {
    tokio::task::spawn_blocking(scanner::scan_temp)
        .await
        .map_err(|e| CommandError { message: e.to_string() })?
        .map_err(Into::into)
}

#[tauri::command]
pub async fn scan_cache() -> CmdResult<ScanResult> {
    tokio::task::spawn_blocking(scanner::scan_cache)
        .await
        .map_err(|e| CommandError { message: e.to_string() })?
        .map_err(Into::into)
}

#[tauri::command]
pub async fn scan_logs() -> CmdResult<ScanResult> {
    tokio::task::spawn_blocking(scanner::scan_logs)
        .await
        .map_err(|e| CommandError { message: e.to_string() })?
        .map_err(Into::into)
}

#[tauri::command]
pub async fn scan_directory(path: String) -> CmdResult<ScanResult> {
    let p = PathBuf::from(path);
    tokio::task::spawn_blocking(move || {
        scanner::scan_directory(&p, quanclean_core::types::ScanCategory::Temp)
    })
    .await
    .map_err(|e| CommandError { message: e.to_string() })?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn clean_files(entries: Vec<FileEntry>) -> CmdResult<CleanResult> {
    tokio::task::spawn_blocking(move || cleaner::clean_files(&entries))
        .await
        .map_err(|e| CommandError { message: e.to_string() })?
        .map_err(Into::into)
}

#[tauri::command]
pub async fn find_duplicates(paths: Vec<String>) -> CmdResult<ScanResult> {
    tokio::task::spawn_blocking(move || {
        let path_bufs: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
        let dirs: Vec<&std::path::Path> = path_bufs.iter().map(|p| p.as_path()).collect();
        duplicates::find_duplicates(&dirs)
    })
    .await
    .map_err(|e| CommandError { message: e.to_string() })?
    .map_err(Into::into)
}
