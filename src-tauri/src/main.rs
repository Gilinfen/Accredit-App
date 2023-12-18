// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod accredit;
mod appinfo;
mod globalstate;
mod utils;
mod verify;

use accredit::{
    create_app_keys, create_signature, download_pub_key, get_app_names, get_verify_signature,
};
use utils::{create_file_if_not_exists, get_app_data_dir};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_signature,
            create_app_keys,
            get_app_names,
            get_verify_signature,
            download_pub_key
        ])
        .setup(|app: &mut tauri::App| {
            // 保存 app 为全局变量
            globalstate::APP_HANDLE
                .set(app.handle().clone())
                .expect("Failed to set app handle");

            let json_path = get_app_data_dir();
            let _ = create_file_if_not_exists(&format!(
                "{}/app_info.json",
                &json_path.to_string_lossy().to_string()
            ));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
