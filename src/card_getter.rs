use crate::constants::BASE_URL;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct Filter<'a> {
    filter_group: FilterGroup<'a>,
}

#[derive(Serialize, Debug)]
pub struct FilterGroup<'a> {
    operator: &'a str,
    filters: Vec<TagFilter<'a>>,
}

#[derive(Serialize, Debug)]
pub struct TagFilter<'a> {
    #[serde(rename = "type")]
    filter_type: &'a str,
    operator: &'a str,
    arg: &'a str,
}

pub async fn get_vocab_card_ids(api_key: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let filter = Filter {
        filter_group: FilterGroup {
            operator: "and",
            filters: vec![TagFilter {
                filter_type: "tag",
                operator: "contains",
                arg: "super-vocab-card",
            }],
        },
    };

    let mut headers = HeaderMap::new();
    headers.insert("Api-Key", HeaderValue::from_str(api_key)?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/cards/get/select", BASE_URL))
        .headers(headers)
        .json(&filter)
        .send()
        .await?;

    let json: HashMap<String, serde_json::Value> = response.json().await?;
    let ids: Vec<String> = json.keys().cloned().collect();
    Ok(ids)
}

#[derive(Serialize)]
pub struct CardRequest {
    specified: Vec<String>,
}

pub async fn get_card_from_id(
    card_id: &str,
    api_key: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let url = format!("{}/cards/get/specify", BASE_URL);
    let card_request = CardRequest {
        specified: vec![card_id.to_string()],
    };

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("accept", "application/json")
        .header("Api-Key", api_key)
        .header("Content-Type", "application/json")
        .json(&card_request)
        .send()
        .await?;

    let json = response.json::<serde_json::Value>().await?;

    Ok(json)
}
