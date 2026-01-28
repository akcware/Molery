mod commands;

use commands::{analyze, clean, optimize, status, uninstall};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            status::check_mo_installed,
            status::get_status,
            status::get_extended_status,
            status::get_home_dir,
            clean::scan_cleanup,
            clean::run_cleanup,
            uninstall::list_apps,
            uninstall::uninstall_app,
            analyze::analyze_path,
            optimize::clear_dns_cache,
            optimize::rebuild_spotlight_index,
            optimize::free_memory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
