import { useTranslation } from "react-i18next";
import type { Page } from "../App";
import i18n from "../i18n";
import styles from "./Sidebar.module.css";

interface SidebarProps {
  currentPage: Page;
  onNavigate: (page: Page) => void;
}

const NAV_ITEMS: { id: Page; icon: string }[] = [
  { id: "dashboard", icon: "⬡" },
  { id: "scan", icon: "◎" },
  { id: "settings", icon: "⚙" },
];

const LANGUAGES = [
  { code: "en", label: "EN" },
  { code: "zh-CN", label: "中文" },
];

export default function Sidebar({ currentPage, onNavigate }: SidebarProps) {
  const { t, i18n: i18nHook } = useTranslation();
  const currentLang = i18nHook.language;

  return (
    <nav className={styles.sidebar}>
      <div className={styles.logo}>
        <span className={styles.logoIcon}>✦</span>
        <span className={styles.logoText}>{t("app.name")}</span>
      </div>

      <ul className={styles.navList}>
        {NAV_ITEMS.map(({ id, icon }) => (
          <li key={id}>
            <button
              className={`${styles.navItem} ${currentPage === id ? styles.active : ""}`}
              onClick={() => onNavigate(id)}
            >
              <span className={styles.navIcon}>{icon}</span>
              <span className={styles.navLabel}>{t(`nav.${id}`)}</span>
            </button>
          </li>
        ))}
      </ul>

      <div className={styles.footer}>
        <div className={styles.langSwitch}>
          {LANGUAGES.map(({ code, label }) => (
            <button
              key={code}
              className={`${styles.langBtn} ${currentLang === code || (code === "zh-CN" && currentLang.startsWith("zh")) ? styles.langActive : ""}`}
              onClick={() => i18n.changeLanguage(code)}
            >
              {label}
            </button>
          ))}
        </div>
        <span className={styles.version}>v0.2.0</span>
      </div>
    </nav>
  );
}
