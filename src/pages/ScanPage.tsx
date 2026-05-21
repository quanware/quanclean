import { useState } from "react";
import { useTranslation } from "react-i18next";
import { useScan } from "../hooks/useScan";
import { formatBytes, formatCount } from "../lib/format";
import type { ScanCategory } from "../lib/types";
import Treemap from "../components/Treemap";
import CategoryList from "../components/CategoryList";
import ConfirmDialog from "../components/ConfirmDialog";
import styles from "./ScanPage.module.css";

export default function ScanPage() {
  const { t } = useTranslation();
  const { scanResult, scanStatus, cleanResult, cleanStatus, error, startScan, startClean, reset } =
    useScan();
  const [showConfirm, setShowConfirm] = useState(false);
  const [selectedCategory, setSelectedCategory] = useState<ScanCategory | null>(null);

  const isScanning = scanStatus === "scanning";
  const isCleaning = cleanStatus === "cleaning";

  const handleClean = async () => {
    setShowConfirm(true);
    await startClean(async () => {
      return new Promise((resolve) => {
        // resolved via ConfirmDialog callbacks
        (window as unknown as { __cleanResolve?: (v: boolean) => void }).__cleanResolve = resolve;
      });
    });
    setShowConfirm(false);
  };

  const confirmClean = () => {
    const resolve = (window as unknown as { __cleanResolve?: (v: boolean) => void }).__cleanResolve;
    if (resolve) {
      resolve(true);
      delete (window as unknown as { __cleanResolve?: (v: boolean) => void }).__cleanResolve;
    }
    setShowConfirm(false);
  };

  const cancelClean = () => {
    const resolve = (window as unknown as { __cleanResolve?: (v: boolean) => void }).__cleanResolve;
    if (resolve) {
      resolve(false);
      delete (window as unknown as { __cleanResolve?: (v: boolean) => void }).__cleanResolve;
    }
    setShowConfirm(false);
  };


  return (
    <div className={styles.page}>
      <header className={styles.header}>
        <div>
          <h1 className={styles.title}>{t("scan.title")}</h1>
          {scanResult && (
            <p className={styles.subtitle}>
              {t("scan.found", { size: formatBytes(scanResult.total_size) })}
              {" · "}
              {formatCount(scanResult.entries.length)} {t("scan.categories.temp").toLowerCase()} files
            </p>
          )}
        </div>

        <div className={styles.actions}>
          <button
            className={styles.scanButton}
            onClick={startScan}
            disabled={isScanning || isCleaning}
          >
            {isScanning ? t("scan.scanning") : t("scan.start")}
          </button>

          {scanResult && scanResult.entries.length > 0 && (
            <button
              className={styles.cleanButton}
              onClick={handleClean}
              disabled={isScanning || isCleaning}
            >
              {isCleaning ? t("clean.cleaning") : t("clean.title")}
            </button>
          )}

          {(cleanResult || error) && (
            <button className={styles.resetButton} onClick={reset}>
              {t("common.close")}
            </button>
          )}
        </div>
      </header>

      {error && (
        <div className={styles.errorBanner}>
          {t("common.error")}: {error}
        </div>
      )}

      {cleanResult && (
        <div className={styles.successBanner}>
          {t("clean.freed", { size: formatBytes(cleanResult.bytes_freed) })}
          {cleanResult.errors.length > 0 && (
            <span className={styles.cleanErrors}> ({cleanResult.errors.length} errors)</span>
          )}
        </div>
      )}

      {isScanning && (
        <div className={styles.scanningState}>
          <div className={styles.spinner} />
          <p>{t("scan.scanning")}</p>
        </div>
      )}

      {scanResult && scanResult.entries.length > 0 && !isScanning && (
        <div className={styles.results}>
          <Treemap result={scanResult} onCategoryClick={setSelectedCategory} />
          <CategoryList
            result={scanResult}
            selectedCategory={selectedCategory}
            onSelectCategory={setSelectedCategory}
          />
        </div>
      )}

      {scanResult && scanResult.entries.length === 0 && !isScanning && (
        <div className={styles.emptyState}>
          <p>{t("scan.complete")}</p>
          <p className={styles.emptySubtext}>Nothing to clean — your disk is tidy!</p>
        </div>
      )}

      {scanStatus === "idle" && !cleanResult && (
        <div className={styles.idleState}>
          <p>{t("scan.title")}</p>
          <p className={styles.idleSubtext}>Press "Start Scan" to analyze your disk.</p>
        </div>
      )}

      {showConfirm && scanResult && (
        <ConfirmDialog
          message={t("clean.confirm", { size: formatBytes(scanResult.total_size) })}
          onConfirm={confirmClean}
          onCancel={cancelClean}
          confirmLabel={t("clean.proceed")}
          cancelLabel={t("clean.cancel")}
        />
      )}
    </div>
  );
}
