use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::constants::BASE_URL;
use crate::key::Key;

#[derive(Serialize, Deserialize, Debug)]
pub struct CardMarkupData {
    pub markup: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardMarkupUpdate {
    pub data: CardMarkupData,
}

pub type CardMarkupUpdates = HashMap<String, CardMarkupUpdate>;

pub struct MarkupUpdater {
    key: Key,
    pub card_ids: Vec<String>,
    pub card_markup_updates: CardMarkupUpdates,
}

impl MarkupUpdater {
    pub fn new(given_card_ids: Vec<String>, updates: CardMarkupUpdates) -> Self {
        let key = Key::new();
        let card_ids = given_card_ids;
        let card_markup_updates = updates;

        MarkupUpdater {
            key,
            card_ids,
            card_markup_updates,
        }
    }

    pub async fn update_markup(&mut self) -> Result<Value, Error> {
        let client = Client::new();
        let response = client
            .patch(format!("{}/cards/", BASE_URL))
            .header("accept", "application/json")
            .header("Api-Key", &self.key.key)
            .header("Content-Type", "application/json")
            .json(&self.card_markup_updates)
            .send()
            .await?;

        println!("Status: {}", response.status());
        let json = response.json::<serde_json::Value>().await?;
        Ok(json)
    }
}
