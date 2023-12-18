use std::fs::OpenOptions;
use std::io::{self};
use std::path::Path;
use std::path::PathBuf;
use tauri::api::path::app_data_dir;

use crate::globalstate::APP_HANDLE;

/// 获取应用数据目录
pub fn get_app_data_dir() -> PathBuf {
    let app_handle: &tauri::AppHandle = APP_HANDLE.get().expect("全局 Tauri App 访问失败");
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
