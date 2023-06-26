// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rsqlite::{Database, Result};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let _idb = instantiate_database();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn instantiate_database() -> Result<()> {
    let database = Database::open("timetrack.db")?;

    database.execute(
        r#"
        create table if not exists timetypes (
            id integer primary key,
            label varchar(45) not null unique,
            defaultHours int(3) not null default 0
        );"#,
        (),
    )?;

    Ok(())
}
