// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mta_timetracking::Timetype;
use rusqlite::{Connection, Error, Result};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_timetypes() -> Result<Vec<Timetype>, String> {
    let all_timetypes = Timetype::get_all();

    match all_timetypes {
        Ok(v) => return Ok(v),
        Err(e) => return Err(e.to_string()),
    }
}

fn main() {
    let _idb = instantiate_database();

    let r = Timetype::get(2);

    println!("Result of timetype get is {:?}", r);

    let tt = Timetype::get_all();

    println!("Result of get_all: {:?}", tt);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![list_timetypes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn instantiate_database() -> Result<()> {
    let database = Connection::open("timetrack.db")?;

    // Add timetypes table
    database.execute(
        "create table if not exists timetypes (
            id integer primary key,
            label varchar(45) not null unique,
            defaultHours int(3) not null default 0
        );",
        (),
    )?;

    // Add users table
    database.execute(
        "create table if not exists users (
            id integer primary key,
            username varchar(100) not null unique,
            displayName varchar(150),
            roles text default '{\"user\"}',
            totals text default '{}'
        );",
        (),
    )?;

    // Add settings table
    database.execute(
        "create table if not exists settings (
            id integer primary,
            settingName varchar(45) not null unique,
            settingValue varchar(255) 
        );",
        (),
    )?;

    // Add entries table.
    database.execute(
        "create table if not exists entries (
            id integer primary key,
            users_id integer not null,
            timetypes_id integer not null,
            time_start datetime not null,
            time_end datetime not null,
            completed tinyint(1) not null default 0,
            foreign key (users_id) references users (id),
            foreign key (timetypes_id) references timetypes (id)
        );",
        (),
    )?;

    Ok(())
}
