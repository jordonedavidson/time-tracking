// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mta_timetracking::Timetype;
use rusqlite::{Connection, Result};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_timetypes() -> Result<Vec<Timetype>, String> {
    let all_timetypes = Timetype::get_all();
    println!("all_timetypes: {:?}", all_timetypes);
    match all_timetypes {
        Ok(v) => {
            println!("Match all_timetypes Ok: {:?}", v);
            return Ok(v);
        }
        Err(e) => {
            println!("Error in list_timetypes: {:?}", e);
            let error_string = format!("{e}");
            return Err(error_string);
        }
    }
}

#[tauri::command]
fn get_timetype(id: i32) -> Result<Timetype, String> {
    println!("getting timetype id {id}");
    let timetype = Timetype::get(id);

    match timetype {
        Ok(t) => return Ok(t),
        Err(e) => {
            let error_string = format!("{e}");
            return Err(error_string);
        }
    }
}

fn main() {
    let _idb = instantiate_database();

    let tt = Timetype::get_all();

    println!("Result of get_all: {:?}", tt);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            list_timetypes,
            get_timetype
        ])
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
