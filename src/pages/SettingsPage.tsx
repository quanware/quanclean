import { useState } from "react";
import { useTranslation } from "react-i18next";
import i18n from "../i18n";
import styles from "./SettingsPage.module.css";

const LANGUAGES = [
  { code: "en", label: "English" },
  { code: "zh-CN", label: "中文 (简体)" },
];

export default function SettingsPage() {
  const { t, i18n: i18nHook } = useTranslation();
  const [lang, setLang] = useState(i18nHook.language);

  const handleLangChange = (code: string) => {
    setLang(code);
    i18n.changeLanguage(code);
  };

  return (
    <div className={styles.page}>
      <h1 className={styles.title}>{t("nav.settings")}</h1>

      <section className={styles.section}>
        <h2 className={styles.sectionTitle}>{t("settings.language")}</h2>
        <div className={styles.langOptions}>
          {LANGUAGES.map(({ code, label }) => (
            <button
              key={code}
              className={`${styles.langOption} ${lang === code || (code === "zh-CN" && lang.startsWith("zh")) ? styles.active : ""}`}
              onClick={() => handleLangChange(code)}
            >
              {label}
            </button>
          ))}
        </div>
      </section>

      <section className={styles.section}>
        <h2 className={styles.sectionTitle}>{t("settings.about")}</h2>
        <p className={styles.aboutText}>
          <strong>{t("app.name")}</strong> — {t("app.tagline")}
        </p>
        <p className={styles.aboutMeta}>
          {t("settings.version", { version: "0.2.0" })} · {t("settings.license")}
        </p>
      </section>
    </div>
  );
}
