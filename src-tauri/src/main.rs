// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod puzzle1;
use crate::puzzle1::puzzle1;
use crate::puzzle1::puzzle1advanced;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, puzzle1, puzzle1advanced])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
