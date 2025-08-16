pub mod db;
pub mod models;
pub mod schema;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    db::run_migrations();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![db::list_dishes, db::list_types])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
