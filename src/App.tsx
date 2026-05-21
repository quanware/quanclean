import { useState } from "react";
import styles from "./App.module.css";
import Sidebar from "./components/Sidebar";
import DashboardPage from "./pages/DashboardPage";
import ScanPage from "./pages/ScanPage";
import SettingsPage from "./pages/SettingsPage";

export type Page = "dashboard" | "scan" | "settings";

export default function App() {
  const [page, setPage] = useState<Page>("dashboard");

  return (
    <div className={styles.layout}>
      <Sidebar currentPage={page} onNavigate={setPage} />
      <main className={styles.content}>
        {page === "dashboard" && <DashboardPage onNavigate={setPage} />}
        {page === "scan" && <ScanPage />}
        {page === "settings" && <SettingsPage />}
      </main>
    </div>
  );
}
