// utils.rs

use serde::Serialize;
use serde_json::{self, Value};
use std::fs::OpenOptions;
use std::io::{self, Read};
use std::path::Path;
use std::{
    fs::{self, File},
    path::PathBuf,
};
use tauri::api::path::app_data_dir;

use crate::globalstate::APP_HANDLE;

/// 获取应用数据目录
pub fn get_app_data_dir() -> PathBuf {
    let app_handle: &tauri::AppHandle = APP_HANDLE.get().expect("全局 Tauri App 访问失败");
    app_data_dir(&app_handle.config()).expect("failed to get app data dir")
}

/// 创建 JSON 文件
pub fn create_file_if_not_exists(file_path: &str) -> io::Result<()> {
    let path = Path::new(file_path);
    println!("{}", file_path);
    if !path.exists() {
        OpenOptions::new().write(true).create(true).open(path)?;
    }
    Ok(())
}

/// 读取 json
pub fn read_json(file_path: &str) -> io::Result<Value> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.trim().is_empty() {
        // 返回一个错误或默认值
        return Ok(serde_json::Value::Null);
    }

    let data: Value = serde_json::from_str(&contents)?;
    Ok(data)
}

/// 修改 json
pub fn update_json<T: Serialize>(data: &T, file_path: &str) -> io::Result<()> {
    let contents = serde_json::to_string(data)?;
    fs::write(file_path, contents)?;
    Ok(())
}
