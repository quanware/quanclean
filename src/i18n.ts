import i18n from "i18next";
import { initReactI18next } from "react-i18next";

import en from "../locales/en/common.json";
import zhCN from "../locales/zh-CN/common.json";

i18n.use(initReactI18next).init({
  resources: {
    en: { common: en },
    "zh-CN": { common: zhCN },
  },
  lng: navigator.language.startsWith("zh") ? "zh-CN" : "en",
  fallbackLng: "en",
  defaultNS: "common",
  interpolation: {
    escapeValue: false,
  },
});

export default i18n;
