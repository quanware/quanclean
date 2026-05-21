mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::scan_full,
            commands::scan_temp,
            commands::scan_cache,
            commands::scan_logs,
            commands::scan_directory,
            commands::clean_files,
            commands::find_duplicates,
        ])
        .run(tauri::generate_context!())
        .expect("error while running QuanClean");
}
