use reqwest::{Client, Error};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::constants::BASE_URL;
use crate::key::Key;

#[derive(Serialize, Deserialize, Debug)]
pub struct CardTagData {
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardTagUpdate {
    pub data: CardTagData,
}

pub type CardTagUpdates = HashMap<String, CardTagUpdate>;

pub struct TagUpdater {
    key: Key,
    pub card_ids: Vec<String>,
    pub card_tag_updates: CardTagUpdates,
}

impl TagUpdater {
    pub fn new(given_card_ids: Vec<String>, updates: CardTagUpdates) -> Self {
        let key = Key::new();
        let card_ids = given_card_ids;
        let card_tag_updates = updates;

        TagUpdater {
            key,
            card_ids,
            card_tag_updates,
        }
    }

    pub async fn update_tags(&mut self) -> Result<Value, Error> {
        let client = Client::new();
        let response = client
            .patch(format!("{}/cards/", BASE_URL))
            .header("accept", "application/json")
            .header("Api-Key", &self.key.key)
            .header("Content-Type", "application/json")
            .json(&self.card_tag_updates)
            .send()
            .await?;
    
        println!("Status: {}", response.status());
        let json = response.json::<serde_json::Value>().await?;
        Ok(json)
    }
}