use once_cell::sync::OnceCell;
use tauri::AppHandle;

pub static APP_HANDLE: OnceCell<AppHandle> = OnceCell::new();
