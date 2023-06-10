use std::fs;

#[derive(Debug)]
pub struct Key {
    pub key: String,
}

impl Key {
    pub fn new() -> Self {
        let contents = fs::read_to_string("./config.json").expect("Unable to read config file");
        let config: serde_json::Value =
            serde_json::from_str(&contents).expect("Unable to parse JSON");
        let key = config["key"]
            .as_str()
            .expect("Unable to get key")
            .to_string();
        Self { key }
    }
}
