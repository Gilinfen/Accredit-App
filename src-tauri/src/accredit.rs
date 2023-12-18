use base64::{engine::general_purpose, Engine as _};
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding},
    Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::AppHandle;
use tokio::task;

use crate::{globalstate::APP_HANDLE, utils, verify};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AppData {
    pub pub_key: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AppInfos {
    pub app_name: AppData,
}

fn get_app_name_path() -> PathBuf {
    utils::get_app_data_dir().join("app_name")
}

fn get_app_info_path() -> String {
    let file_path = utils::get_app_data_dir();
    format!("{}/app_info.json", &file_path.to_string_lossy().to_string())
}

fn read_json_command() -> Result<Value, String> {
    let app_info_path = get_app_info_path();
    utils::read_json(&app_info_path).map_err(|e| e.to_string())
}

fn update_json_command(data: AppInfos) -> Result<(), String> {
    let app_info_path = get_app_info_path();
    utils::update_json(&data, &app_info_path).map_err(|e| e.to_string())
}

// 使用私钥对数据进行签名
async fn sign_data(
    priv_key: Arc<RsaPrivateKey>,
    data: Vec<u8>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let signature = task::spawn_blocking(move || {
        let mut hasher = Sha256::new();
        hasher.update(data); // 对数据进行哈希处理
        let hashed_data = hasher.finalize();

        // 对哈希后的数据进行签名
        priv_key
            .sign(Pkcs1v15Sign::new_unprefixed(), &hashed_data)
            .expect("failed to sign data")
    })
    .await?;

    Ok(signature)
}

// 生成 RSA 密钥对（公钥和私钥）
async fn generate_key_pair(app_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let _ = task::spawn_blocking(move || {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);
        // 存储私钥
        let _ = RsaPrivateKey::write_pkcs8_pem_file(
            &priv_key,
            format!("{}/private_key.pem", &app_name),
            LineEnding::LF,
        );
        // // 读取私钥
        // let priv_key_pem = RsaPrivateKey::read_pkcs8_pem_file("private_key.pem").expect("msg");
        // 存储公钥
        let _ = RsaPublicKey::write_public_key_pem_file(
            &pub_key,
            format!("{}/public_key.pem", &app_name),
            LineEnding::LF,
        );
        println!(
            "---------------{}",
            &format!("{}/public_key.pem", &app_name)
        );
        // // 读取公钥
        // let pub_key_pem = RsaPublicKey::read_public_key_pem_file("public_key.pem").expect("msg");
    })
    .await?;

    Ok(())
}

// 生成 RSA 密钥对
#[tauri::command]
pub async fn create_app_keys(app_name: String) -> Result<(), String> {
    let app_data_path = get_app_name_path();

    let app_data_name = app_data_path.join(&app_name);

    let _ = fs::create_dir_all(&app_data_name);

    let _ = generate_key_pair(app_data_name.to_string_lossy().to_string()).await;
    Ok(())
}

// 异步生成签名
#[tauri::command]
pub async fn create_signature(data: Vec<u8>, app_name: &str) -> Result<String, String> {
    let app_data_path = get_app_name_path();

    let app_data_name = app_data_path.join(format!("{}/private_key.pem", app_name));

    // 读取私钥
    let priv_key_pem = RsaPrivateKey::read_pkcs8_pem_file(&app_data_name).expect("msg");

    let signature = sign_data(Arc::new(priv_key_pem), data)
        .await
        .map_err(|e| e.to_string())?;
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(&signature);

    Ok(encoded)
}

// 获取应用名
#[tauri::command]
pub fn get_app_names(app_handle: AppHandle) -> Result<Vec<String>, String> {
    let app_data_path: PathBuf = get_app_name_path();

    get_filenames_in_directory(&app_data_path).map_err(|e| e.to_string()) // 转换错误为 String
}

fn get_filenames_in_directory(directory: &Path) -> io::Result<Vec<String>> {
    let mut filenames = Vec::new();

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                filenames.push(name.to_owned());
            }
        }
    }

    Ok(filenames)
}

// 验证签名
#[tauri::command]
pub fn get_verify_signature(
    app_handle: AppHandle,
    app_name: &str,
    user_data: Vec<u8>,
    signature: Vec<u8>,
) -> Result<bool, String> {
    let app_data_path: PathBuf = get_app_name_path();

    // 读取公钥
    let pub_key_pem = RsaPublicKey::read_public_key_pem_file(
        app_data_path.join(format!("{}/public_key.pem", app_name)),
    )
    .expect("获取公要失败");

    let vals = verify::verify_signature(&pub_key_pem, &user_data, &signature);

    Ok(vals)
}

// 下载公钥
#[tauri::command]
pub fn download_pub_key(
    app_handle: AppHandle,
    app_name: &str,
    new_path: &str,
) -> Result<(), String> {
    let app_data_path: PathBuf = get_app_name_path();

    let keypath = app_data_path.join(format!("{}/public_key.pem", &app_name));

    fs::copy(&keypath, new_path).map_err(|e| e.to_string())?;

    Ok(())
}
