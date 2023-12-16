use rand::rngs::OsRng;
use rsa::{PaddingScheme, PublicKey, PublicKeyEncoding, RSAPrivateKey, RSAPublicKey};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Write;

// 生成 RSA 密钥对（公钥和私钥）
fn generate_key_pair() -> (RSAPrivateKey, RSAPublicKey) {
    let mut rng = OsRng; // 随机数生成器
    let bits = 2048; // 密钥长度
    let priv_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key: RSAPublicKey = RSAPublicKey::from(&priv_key);
    (priv_key, pub_key)
}

// 使用私钥对数据进行签名
fn sign_data(priv_key: &RSAPrivateKey, data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new(); // 创建 SHA-256 哈希实例
    hasher.update(data); // 对数据进行哈希处理
    let hashed_data = hasher.finalize();

    // 对哈希后的数据进行签名
    priv_key
        .sign(PaddingScheme::new_pkcs1v15_sign(None), &hashed_data)
        .expect("failed to sign data")
}

fn save_public_key_to_pem(pub_key: &RSAPublicKey, filename: &str) -> Result<(), std::io::Error> {
    let pem = pem::encode(&pem::Pem {
        tag: String::from("RSA PUBLIC KEY"),
        contents: pub_key.to_pkcs1().expect("Failed to convert key"),
    });

    let mut file = File::create(filename)?;
    file.write_all(pem.as_bytes())?;

    Ok(())
}

fn main(data: &[u8]) {
    let (priv_key, pub_key) = generate_key_pair();

    save_public_key_to_pem(&pub_key, "public_key.pem").expect("Failed to save public key");

    // 要签名的数据
    // let data: &[u8; 13] = b"Hello, world!";

    // 生成签名
    let signature: Vec<u8> = sign_data(&priv_key, data);
}
