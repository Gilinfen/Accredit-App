use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Signature {
    pub base_code: String,
    pub use_info: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AppInfo {
    pub app_name: String,
    pub app_name_path: String,
    pub pri_key_path: String,
    pub pub_key_puth: String,
    pub signature: Vec<Signature>,
}
impl AppInfo {
    // 添加签名到 signature 数组
    pub fn add_signature(&mut self, signature: Signature) {
        self.signature.push(signature);
    }
}

pub fn read_or_create_json(file_path: &str) -> io::Result<Vec<AppInfo>> {
    if Path::new(file_path).exists() {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        if contents.trim().is_empty() {
            Ok(Vec::new())
        } else {
            serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        }
    } else {
        Ok(Vec::new())
    }
}

pub fn add_element_and_save(file_path: &str, element: AppInfo) -> io::Result<()> {
    let mut objects = read_or_create_json(file_path)?;
    objects.push(element);
    let json_string = serde_json::to_string(&objects)?;
    fs::write(file_path, json_string)?;
    Ok(())
}

// fn main() -> io::Result<()> {
//     let file_path = "path/to/your/file.json";

//     let new_element = AppInfo {
//         field1: "example".to_string(),
//         field2: 123,
//         // 初始化其他字段...
//     };

//     add_element_and_save(file_path, new_element)?;

//     Ok(())
// }
