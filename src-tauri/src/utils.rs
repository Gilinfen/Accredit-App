use base64::{engine::general_purpose, Engine as _};
use std::fs::OpenOptions;
use std::io::{self};
use std::path::Path;
use std::path::PathBuf;
use tauri::api::path::app_data_dir;
use tauri::AppHandle;

/// 获取应用数据目录
pub fn get_app_data_dir(app_handle: &AppHandle) -> PathBuf {
    app_data_dir(&app_handle.config()).expect("failed to get app data dir")
}

/// 创建文件
pub fn create_file_if_not_exists(file_path: &str) -> io::Result<()> {
    let path = Path::new(file_path);
    if !path.exists() {
        OpenOptions::new().write(true).create(true).open(path)?;
    }
    Ok(())
}

// base 解码
#[tauri::command]
pub fn decode_str(string: &str) -> Vec<u8> {
    let decoded_bytes = general_purpose::STANDARD_NO_PAD
        .decode(&string.as_bytes().to_vec())
        .expect("解码失败");
    let pem_str = std::str::from_utf8(&decoded_bytes);
    match pem_str {
        Ok(p) => println!("sasd. {}", p),
        Err(e) => println!("无法转换为UTF-8字符串：{}", e),
    };
    decoded_bytes
}

// base 编码
#[tauri::command]
pub fn encode_str(string: String) -> String {
    let decoded_bytes = general_purpose::STANDARD_NO_PAD.encode(&string);

    decoded_bytes
}
