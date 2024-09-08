// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use migration::sea_orm::DbConn;

mod database;
mod helper;

#[tokio::main]
async fn main() {
    let db: DbConn = database::init_database();
    let state = AppState { db };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_sett
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Clone)]
struct AppState {
    db: DbConn,
}