// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod puzzle1;
mod puzzle2;
mod puzzle3;
mod puzzle4;

use crate::puzzle1::puzzle1;
use crate::puzzle1::puzzle1advanced;
use crate::puzzle2::puzzle2;
use crate::puzzle2::puzzle2advanced;
use crate::puzzle3::puzzle3;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            puzzle1,
            puzzle1advanced,
            puzzle2,
            puzzle2advanced,
            puzzle3
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
