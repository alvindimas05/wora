use std::fs;
use migration::sea_orm::{Database, DbConn};

pub async fn init_database() -> DbConn {
    let home_dir = match tauri::api::path::home_dir() {
        Some(val) => val,
        None => panic!("Could not get home directory."),
    };
    let data_dir = home_dir.join(".wora");
    if let Err(_) = fs::metadata(&data_dir) {
        fs::create_dir_all(&data_dir).expect("Could not create data directory.");
    }
    Database::connect("sqlite:/Volumes/SIGMA/Projects/wora/src-tauri/wora.db?mode=rwc").await
        .expect("Database connection failed.")
}