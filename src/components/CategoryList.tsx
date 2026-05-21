import { useTranslation } from "react-i18next";
import type { ScanResult, ScanCategory } from "../lib/types";
import { formatBytes } from "../lib/format";
import styles from "./CategoryList.module.css";

interface CategoryListProps {
  result: ScanResult;
  selectedCategory: ScanCategory | null;
  onSelectCategory: (cat: ScanCategory | null) => void;
}

const CATEGORIES: ScanCategory[] = ["temp", "cache", "logs", "duplicates", "residue"];

export default function CategoryList({ result, selectedCategory, onSelectCategory }: CategoryListProps) {
  const { t } = useTranslation();

  const displayedEntries = selectedCategory
    ? result.entries.filter((e) => e.category === selectedCategory)
    : result.entries;

  return (
    <div className={styles.container}>
      <div className={styles.filters}>
        <button
          className={`${styles.filterChip} ${selectedCategory === null ? styles.active : ""}`}
          onClick={() => onSelectCategory(null)}
        >
          All ({result.entries.length})
        </button>
        {CATEGORIES.map((cat) => {
          const count = result.entries.filter((e) => e.category === cat).length;
          if (count === 0) return null;
          return (
            <button
              key={cat}
              className={`${styles.filterChip} ${selectedCategory === cat ? styles.active : ""}`}
              onClick={() => onSelectCategory(selectedCategory === cat ? null : cat)}
            >
              {t(`scan.categories.${cat}`)} ({count})
            </button>
          );
        })}
      </div>

      <div className={styles.list}>
        {displayedEntries.slice(0, 500).map((entry, i) => (
          <div key={`${entry.path}-${i}`} className={styles.entry}>
            <span className={styles.entryPath}>{entry.path}</span>
            <span className={styles.entrySize}>{formatBytes(entry.size)}</span>
          </div>
        ))}
        {displayedEntries.length > 500 && (
          <div className={styles.overflow}>
            +{displayedEntries.length - 500} more files
          </div>
        )}
      </div>
    </div>
  );
}
