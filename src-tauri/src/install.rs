use std::fs::{self, File};
use std::io::Error;
use std::path::PathBuf;
use tauri::api::path::app_data_dir;

fn get_app_data_flag_path(config: &tauri::Config, path: &str) -> PathBuf {
    let app_data_dir = app_data_dir(config).expect("failed to get app data dir");
    app_data_dir.join(path)
}

pub fn is_first_run(config: &tauri::Config) -> bool {
    !get_app_data_flag_path(config, "installed.flag").exists()
}

pub fn create_app_data_flag(config: &tauri::Config, path: &str) -> Result<PathBuf, Error> {
    let flag_path: PathBuf = get_app_data_flag_path(config, path);
    if let Some(parent) = flag_path.parent() {
        fs::create_dir_all(parent)?;
    }
    File::create(&flag_path)?;
    Ok(flag_path)
}
