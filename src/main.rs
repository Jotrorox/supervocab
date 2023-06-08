use serde::{Serialize, Deserialize};
use std::fs;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;

const BASE_URL: &str = "https://api.supernotes.app/v1";

#[derive(Debug)]
struct Key {
    key: String,
}

impl Key {
    fn new() -> Self {
        let contents = fs::read_to_string("./config.json").expect("Unable to read config file");
        let config: serde_json::Value = serde_json::from_str(&contents).expect("Unable to parse JSON");
        let key = config["key"].as_str().expect("Unable to get key").to_string();
        Self { key }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SimpleCard {
    name: String,
    markup: String,
    tags: Option<Vec<String>>,
    id: Option<String>,
}

impl SimpleCard {
    fn new(name: &str, markup: &str, tags: Option<Vec<String>>) -> Self {
        Self {
            name: name.to_string(),
            markup: markup.to_string(),
            tags,
            id: None,
        }
    }

    fn add_tag(&mut self, tag: &str) {
        if let Some(tags) = &mut self.tags {
            tags.push(tag.to_string());
        } else {
            self.tags = Some(vec![tag.to_string()]);
        }
    }

    // fn set_id(&mut self, id: &str) {
    //     self.id = Some(id.to_string());
    // }

    async fn send(&self, key: &Key) -> Result<(), reqwest::Error> {
        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert("Api-Key", HeaderValue::from_str(&key.key).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let body = serde_json::to_string(&self).expect("Unable to serialize SimpleCard");

        client
            .post(&format!("{}/cards/simple", BASE_URL))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let key = Key::new();

    let mut card = SimpleCard::new("Test Card", "This is a test card", None);
    card.add_tag("api-test");

    if let Err(e) = card.send(&key).await {
        eprintln!("Error sending card: {}", e);
    }
}
