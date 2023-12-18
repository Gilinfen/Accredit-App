use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct YourObject {
    app_name: String,
    pub_key: String,
}

fn read_or_create_json(file_path: &str) -> io::Result<Vec<YourObject>> {
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

fn add_element_and_save(file_path: &str, element: YourObject) -> io::Result<()> {
    let mut objects = read_or_create_json(file_path)?;
    objects.push(element);
    let json_string = serde_json::to_string(&objects)?;
    fs::write(file_path, json_string)?;
    Ok(())
}

// fn main() -> io::Result<()> {
//     let file_path = "path/to/your/file.json";

//     let new_element = YourObject {
//         field1: "example".to_string(),
//         field2: 123,
//         // 初始化其他字段...
//     };

//     add_element_and_save(file_path, new_element)?;

//     Ok(())
// }
