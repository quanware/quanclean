import { invoke } from "@tauri-apps/api/core";
import type { ScanResult, CleanResult, FileEntry } from "./types";

export async function scanFull(): Promise<ScanResult> {
  return invoke<ScanResult>("scan_full");
}

export async function scanTemp(): Promise<ScanResult> {
  return invoke<ScanResult>("scan_temp");
}

export async function scanCache(): Promise<ScanResult> {
  return invoke<ScanResult>("scan_cache");
}

export async function scanLogs(): Promise<ScanResult> {
  return invoke<ScanResult>("scan_logs");
}

export async function scanDirectory(path: string): Promise<ScanResult> {
  return invoke<ScanResult>("scan_directory", { path });
}

export async function cleanFiles(entries: FileEntry[]): Promise<CleanResult> {
  return invoke<CleanResult>("clean_files", { entries });
}

export async function findDuplicates(paths: string[]): Promise<ScanResult> {
  return invoke<ScanResult>("find_duplicates", { paths });
}
