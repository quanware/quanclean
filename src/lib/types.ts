export type ScanCategory = "temp" | "cache" | "logs" | "duplicates" | "residue";

export interface FileEntry {
  path: string;
  size: number;
  category: ScanCategory;
  hash: string | null;
}

export interface ScanResult {
  entries: FileEntry[];
  total_size: number;
}

export interface CleanResult {
  files_deleted: number;
  bytes_freed: number;
  errors: string[];
}

export type ScanStatus = "idle" | "scanning" | "done" | "error";
export type CleanStatus = "idle" | "confirming" | "cleaning" | "done" | "error";
