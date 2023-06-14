use crate::constants::BASE_URL;
use crate::key::Key;
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleCard {
    pub name: String,
    pub markup: String,
    pub tags: Option<Vec<String>>,
    pub id: Option<String>,
}

impl SimpleCard {
    #[allow(dead_code)]
    pub fn new(name: &str, markup: &str, tags: Option<Vec<String>>) -> Self {
        Self {
            name: name.to_string(),
            markup: markup.to_string(),
            tags,
            id: None,
        }
    }

    #[allow(dead_code)]
    pub fn add_tag(&mut self, tag: &str) {
        if let Some(tags) = &mut self.tags {
            tags.push(tag.to_string());
        } else {
            self.tags = Some(vec![tag.to_string()]);
        }
    }

    #[allow(dead_code)]
    pub fn set_id(&mut self, id: &str) {
        self.id = Some(id.to_string());
    }

    #[allow(dead_code)]
    pub async fn send(&self, key: &Key) -> Result<(), reqwest::Error> {
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
