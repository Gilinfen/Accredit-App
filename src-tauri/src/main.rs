// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod accredit;
mod verify;

use accredit::create_signature;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_signature])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
