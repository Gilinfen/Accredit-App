use base64::{engine::general_purpose, Engine as _};
use rand::rngs::OsRng;
use rsa::hash::Hash;
use rsa::{PaddingScheme, RSAPrivateKey, RSAPublicKey};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::task;

// 生成 RSA 密钥对（公钥和私钥）
async fn generate_key_pair() -> Result<(RSAPrivateKey, RSAPublicKey), Box<dyn std::error::Error>> {
    let key_pair = task::spawn_blocking(|| {
        let mut rng = OsRng; // 随机数生成器
        let bits = 2048; // 密钥长度
        let priv_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key: RSAPublicKey = RSAPublicKey::from(&priv_key);
        (priv_key, pub_key)
    })
    .await?;

    Ok(key_pair)
}

// 使用私钥对数据进行签名
async fn sign_data(
    priv_key: Arc<RSAPrivateKey>,
    data: Vec<u8>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let signature = task::spawn_blocking(move || {
        let mut hasher = Sha256::new(); // 创建 SHA-256 哈希实例
        hasher.update(&data); // 对数据进行哈希处理
        let hashed_data = &hasher.finalize() as &[u8];

        // 对哈希后的数据进行签名
        priv_key
            .sign(
                PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),
                &hashed_data,
            )
            .expect("failed to sign data")
    })
    .await?;

    Ok(signature)
}

#[tauri::command]
pub async fn create_signature(data: Vec<u8>) -> Result<String, String> {
    println!("datadatadata{:?}", data);
    // 异步生成密钥对
    let key_pair = generate_key_pair().await.map_err(|e| e.to_string())?;
    let (priv_key, pub_key) = key_pair;

    // 异步生成签名
    let signature = sign_data(Arc::new(priv_key), data)
        .await
        .map_err(|e| e.to_string())?;
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(&signature);

    Ok(encoded)
}
