import { useTranslation } from "react-i18next";
import type { Page } from "../App";
import styles from "./DashboardPage.module.css";

interface DashboardPageProps {
  onNavigate: (page: Page) => void;
}

const FEATURES = [
  { key: "temp", color: "var(--color-temp)" },
  { key: "cache", color: "var(--color-cache)" },
  { key: "logs", color: "var(--color-logs)" },
  { key: "duplicates", color: "var(--color-duplicates)" },
  { key: "residue", color: "var(--color-residue)" },
];

export default function DashboardPage({ onNavigate }: DashboardPageProps) {
  const { t } = useTranslation();

  return (
    <div className={styles.page}>
      <header className={styles.header}>
        <h1 className={styles.title}>{t("app.name")}</h1>
        <p className={styles.tagline}>{t("app.tagline")}</p>
      </header>

      <div className={styles.hero}>
        <div className={styles.heroIcon}>✦</div>
        <p className={styles.heroText}>
          {t("scan.found", { size: "—" })}
        </p>
        <button className={styles.ctaButton} onClick={() => onNavigate("scan")}>
          {t("scan.start")}
        </button>
      </div>

      <section className={styles.categories}>
        {FEATURES.map(({ key, color }) => (
          <div key={key} className={styles.categoryCard} style={{ "--cat-color": color } as React.CSSProperties}>
            <span className={styles.categoryDot} />
            <span className={styles.categoryLabel}>{t(`scan.categories.${key}`)}</span>
          </div>
        ))}
      </section>
    </div>
  );
}
