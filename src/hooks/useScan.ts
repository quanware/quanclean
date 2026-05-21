import { useState, useCallback } from "react";
import type { ScanResult, CleanResult, ScanStatus, CleanStatus } from "../lib/types";
import * as api from "../lib/tauri";

interface UseScanReturn {
  scanResult: ScanResult | null;
  scanStatus: ScanStatus;
  cleanResult: CleanResult | null;
  cleanStatus: CleanStatus;
  error: string | null;
  startScan: () => Promise<void>;
  startClean: (confirmCallback?: () => Promise<boolean>) => Promise<void>;
  reset: () => void;
}

export function useScan(): UseScanReturn {
  const [scanResult, setScanResult] = useState<ScanResult | null>(null);
  const [scanStatus, setScanStatus] = useState<ScanStatus>("idle");
  const [cleanResult, setCleanResult] = useState<CleanResult | null>(null);
  const [cleanStatus, setCleanStatus] = useState<CleanStatus>("idle");
  const [error, setError] = useState<string | null>(null);

  const startScan = useCallback(async () => {
    setScanStatus("scanning");
    setError(null);
    setScanResult(null);
    setCleanResult(null);
    setCleanStatus("idle");
    try {
      const result = await api.scanFull();
      setScanResult(result);
      setScanStatus("done");
    } catch (e) {
      setError(String(e));
      setScanStatus("error");
    }
  }, []);

  const startClean = useCallback(
    async (confirmCallback?: () => Promise<boolean>) => {
      if (!scanResult || scanResult.entries.length === 0) return;

      if (confirmCallback) {
        setCleanStatus("confirming");
        const confirmed = await confirmCallback();
        if (!confirmed) {
          setCleanStatus("idle");
          return;
        }
      }

      setCleanStatus("cleaning");
      setError(null);
      try {
        const result = await api.cleanFiles(scanResult.entries);
        setCleanResult(result);
        setCleanStatus("done");
        // Refresh scan result to reflect deleted files
        setScanResult(null);
        setScanStatus("idle");
      } catch (e) {
        setError(String(e));
        setCleanStatus("error");
      }
    },
    [scanResult]
  );

  const reset = useCallback(() => {
    setScanResult(null);
    setScanStatus("idle");
    setCleanResult(null);
    setCleanStatus("idle");
    setError(null);
  }, []);

  return { scanResult, scanStatus, cleanResult, cleanStatus, error, startScan, startClean, reset };
}
