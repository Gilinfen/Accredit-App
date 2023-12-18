use base64::{engine::general_purpose, Engine as _};
use rsa::{Pkcs1v15Sign, RsaPublicKey};
use sha2::{Digest, Sha256};

// 使用公钥验证签名
pub fn verify_signature(pub_key: &RsaPublicKey, data: &[u8], signature: &[u8]) -> bool {
    let dencoed = general_purpose::STANDARD_NO_PAD.decode(&signature);

    match dencoed {
        Ok(dencoed_val) => {
            let mut hasher = Sha256::new(); // 创建 SHA-256 哈希实例
            hasher.update(data); // 对数据进行哈希处理
            let hashed_data = hasher.finalize();

            // 验证签名
            pub_key
                .verify(Pkcs1v15Sign::new_unprefixed(), &hashed_data, &dencoed_val)
                .is_ok()
        }
        Err(_) => false,
    }
}
