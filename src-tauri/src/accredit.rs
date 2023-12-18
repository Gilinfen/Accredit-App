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
use tokio::task;

use crate::{
    appinfo::{self, AppInfo, Signature},
    utils, verify,
};

fn get_app_name_path() -> PathBuf {
    utils::get_app_data_dir().join("app_name")
}

fn get_app_info_path() -> String {
    let file_path = utils::get_app_data_dir();
    format!("{}/app_info.json", &file_path.to_string_lossy().to_string())
}

fn read_json_command() -> io::Result<Vec<AppInfo>> {
    let app_info_path = get_app_info_path();
    appinfo::read_or_create_json(&app_info_path)
}

fn update_json_command(data: AppInfo) -> io::Result<()> {
    let app_info_path = get_app_info_path();
    appinfo::add_element_and_save(&app_info_path, data)
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
async fn generate_key_pair(
    app_name_path: String,
    app_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = task::spawn_blocking(move || {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);

        let pri_key_path = format!("{}/private_key.pem", &app_name_path);
        let pub_key_puth = format!("{}/public_key.pem", &app_name_path);

        // 存储私钥
        let _ = RsaPrivateKey::write_pkcs8_pem_file(&priv_key, &pri_key_path, LineEnding::LF);
        // // 读取私钥
        // let priv_key_pem = RsaPrivateKey::read_pkcs8_pem_file("private_key.pem").expect("msg");
        // 存储公钥
        let _ = RsaPublicKey::write_public_key_pem_file(&pub_key, &pub_key_puth, LineEnding::LF);

        // 初始化应用签名信息
        let signature = vec![];

        // 存储应用信息
        let new_element = AppInfo {
            app_name,
            app_name_path,
            pri_key_path,
            pub_key_puth,
            signature,
        };

        let _ = update_json_command(new_element);
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

    let _ = generate_key_pair(app_data_name.to_string_lossy().to_string(), app_name).await;
    Ok(())
}

// 异步生成签名
#[tauri::command]
pub async fn create_signature(data: Vec<u8>, app_name: &str) -> Result<String, String> {
    let app_data_path = get_app_name_path();

    let app_data_name = app_data_path.join(format!("{}/private_key.pem", app_name));

    // 读取私钥
    let priv_key_pem = RsaPrivateKey::read_pkcs8_pem_file(&app_data_name).expect("msg");

    let signature = sign_data(Arc::new(priv_key_pem), data.clone())
        .await
        .map_err(|e| e.to_string())?;
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(&signature);

    let signature_val = Signature {
        base_code: encoded.clone(),
        use_info: String::from_utf8(data).expect("from_utf8 转换失败"),
    };

    let mut app_info_json = read_json_command().expect("获取 JSON 失败");

    if let Some(app_info_json) = app_info_json.iter_mut().find(|s| s.app_name == app_name) {
        app_info_json.add_signature(signature_val)
    }

    Ok(encoded)
}

// 获取应用名
#[tauri::command]
pub fn get_app_names() -> Result<Vec<String>, String> {
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
pub fn download_pub_key(app_name: &str, new_path: &str) -> Result<(), String> {
    let app_data_path: PathBuf = get_app_name_path();

    let keypath = app_data_path.join(format!("{}/public_key.pem", &app_name));

    fs::copy(&keypath, new_path).map_err(|e| e.to_string())?;

    Ok(())
}
