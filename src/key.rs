use std::fs::{self, File};
use std::io::Write;

#[derive(Debug)]
pub struct Key {
    pub key: String,
}

impl Key {
    pub fn new() -> Self {
        if !fs::metadata("./config.json").is_ok() {
            let key: String;
            println!("Enter your key: ");
            key = rpassword::read_password()
                .unwrap()
                .chars()
                .collect::<String>();
            File::create("./config.json")
                .unwrap()
                .write_all(format!(r#"{{"key": "{}"}}"#, key).as_bytes())
                .unwrap();
            return Self { key };
        } else {
            let contents = fs::read_to_string("./config.json").expect("Unable to read config file");
            let config: serde_json::Value =
                serde_json::from_str(&contents).expect("Unable to parse JSON");
            let key = config["key"]
                .as_str()
                .expect("Unable to get key")
                .to_string();
            return Self { key };
        }        
    }
}
