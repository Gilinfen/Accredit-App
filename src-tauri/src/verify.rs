use base64::{engine::general_purpose, Engine as _};
use rsa::{PaddingScheme, PublicKey, RSAPublicKey};
use sha2::{Digest, Sha256};

/// 使用公钥验证签名
///
/// rsa = "0.4.0"                                          # 使用 rsa 库，版本 0.4.0
///
/// sha2 = "0.9.0"                                         # 用于哈希的 sha2 库
///
/// base64 = "0.21.5"
///
pub fn verify_signature(pub_key: &RSAPublicKey, data: &[u8], signature: &str) -> bool {
    let dencoed = general_purpose::STANDARD_NO_PAD.decode(&signature).unwrap();

    let mut hasher = Sha256::new(); // 创建 SHA-256 哈希实例
    hasher.update(data); // 对数据进行哈希处理
    let hashed_data = &hasher.finalize() as &[u8];

    // 验证签名
    pub_key
        .verify(
            PaddingScheme::new_pkcs1v15_sign(None),
            &hashed_data,
            &dencoed,
        )
        .is_ok()
}
