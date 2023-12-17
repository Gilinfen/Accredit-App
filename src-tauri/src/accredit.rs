use base64::{engine::general_purpose, Engine as _};
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding},
    Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey,
};
use sha2::{Digest, Sha256};
use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::api::path::app_data_dir;
use tauri::AppHandle;
use tokio::task;

use crate::verify::verify_signature;

fn get_app_data_dir(app_handle: &AppHandle) -> PathBuf {
    app_data_dir(&app_handle.config())
        .expect("failed to get app data dir")
        .join("app_name")
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
        )
        // // 读取公钥
        // let pub_key_pem = RsaPublicKey::read_public_key_pem_file("public_key.pem").expect("msg");
    })
    .await?;

    Ok(())
}

// 生成 RSA 密钥对
#[tauri::command]
pub async fn create_app_keys(app_handle: AppHandle, app_name: String) -> Result<(), String> {
    let app_data_path = get_app_data_dir(&app_handle);

    let app_data_name = app_data_path.join(&app_name);

    let _ = fs::create_dir_all(&app_data_name);

    let _ = generate_key_pair(app_data_name.to_string_lossy().to_string()).await;
    Ok(())
}

// 异步生成签名
#[tauri::command]
pub async fn create_signature(
    app_handle: AppHandle,
    data: Vec<u8>,
    app_name: &str,
) -> Result<String, String> {
    let app_data_path = get_app_data_dir(&app_handle);

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
    let app_data_path: PathBuf = get_app_data_dir(&app_handle);

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
    data: &[u8],
    signature: &[u8],
) -> Result<bool, String> {
    println!("SUUCESS");
    let app_data_path: PathBuf = get_app_data_dir(&app_handle);

    // 读取公钥
    let pub_key_pem = RsaPublicKey::read_public_key_pem_file(
        app_data_path.join(format!("{}/public_key.pem", app_name)),
    )
    .expect("获取公要失败");

    let vals = verify_signature(&pub_key_pem, data, signature);

    Ok(vals)
}
