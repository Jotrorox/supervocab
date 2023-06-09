use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use chrono::{DateTime, Utc};

mod simple_card;

use simple_card::SimpleCard;

mod constants;

use constants::BASE_URL;

mod key;

use key::Key;

#[derive(Serialize, Debug)]
struct Filter<'a> {
    filter_group: FilterGroup<'a>,
}

#[derive(Serialize, Debug)]
struct FilterGroup<'a> {
    operator: &'a str,
    filters: Vec<TagFilter<'a>>,
}

#[derive(Serialize, Debug)]
struct TagFilter<'a> {
    #[serde(rename = "type")]
    filter_type: &'a str,
    operator: &'a str,
    arg: &'a str,
}

async fn get_vocab_card_ids(api_key: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
struct CardRequest {
    specified: Vec<String>,
}

async fn get_card_from_id(card_id: &str, api_key: &str) -> Result<Value, Box<dyn std::error::Error>> {
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

#[derive(Serialize, Deserialize, Debug)]

struct CardTags {
    tags: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct TagUpdateCardData {
    tags: CardTags,
}

#[derive(Serialize, Deserialize, Debug)]
struct TagUpdateCard {
    id: String,
    data: TagUpdateCardData,
}

impl TagUpdateCard {
    fn new(id: &str, tags: Vec<String>) -> Self {
        Self {
            id: id.to_string(),
            data: TagUpdateCardData {
                tags: CardTags {
                    tags,
                }
            }
        }
    }
    async fn update_tags(&self, api_key: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client
            .patch(&format!("{}/cards/", BASE_URL))
            .header("accept", "application/json")
            .header("Api-Key", api_key)
            .header("Content-Type", "application/json")
            .json(&self)
            .send()
            .await?;
    
        let updated_card = response.json::<serde_json::Value>().await?;
        Ok(updated_card)
    }
}

async fn check_if_due(mut resp: Value, card_id: &str, api_key: &str) -> bool {
    if resp[card_id]["membership"]["status"] != 2 { 
        return false;
    }

    if &resp[card_id]["data"]["tags"].clone().to_owned().as_array_mut().unwrap().len() <= &1 {
        let tuc = TagUpdateCard::new(card_id, vec!["super-vocab-stage-0".to_string()]);
        match tuc.update_tags(api_key).await {
            Ok(tuc_resp) => resp = tuc_resp,
            Err(e) => eprintln!("Error getting card ids: {}", e),
        }
    }
    println!("resp: {}", resp);
    let mut given_time_string = format!("{}Z", resp[card_id]["data"]["modified_when"]);
    given_time_string.retain(|c| c != '\"');
    let given_time = DateTime::parse_from_rfc3339(
        &given_time_string,
    ).unwrap().with_timezone(&Utc);
    let current_time = Utc::now();
    let duration = current_time - given_time;
    let days_difference = duration.num_days();

    let tags = resp[card_id]["data"]["tags"].as_array().unwrap();

    if tags.contains(&serde_json::Value::String("super-vocab-stage-0".to_string())) {
        return true
    } else if tags.contains(&serde_json::Value::String("super-vocab-stage-1".to_string())) {
        if days_difference >= 1 {
            return true
        }
    } else if tags.contains(&serde_json::Value::String("super-vocab-stage-2".to_string())) {
        if days_difference >= 3 {
            return true
        }
    } else if tags.contains(&serde_json::Value::String("super-vocab-stage-3".to_string())) {
        if days_difference >= 7 {
            return true
        }
    } else if tags.contains(&serde_json::Value::String("super-vocab-stage-4".to_string())) {
        if days_difference >= 14 {
            return true
        }
    } else if tags.contains(&serde_json::Value::String("super-vocab-stage-5".to_string())) {
        if days_difference >= 30 {
            return true
        }
    } else if tags.contains(&serde_json::Value::String("super-vocab-stage-6".to_string())) {
        if days_difference >= 90 {
            return true
        }
    } else if tags.contains(&serde_json::Value::String("super-vocab-stage-7".to_string())) {
        if days_difference >= 180 {
            return true
        }
    } else if tags.contains(&serde_json::Value::String("super-vocab-stage-8".to_string())) {
        if days_difference >= 365 {
            return true
        }
    } 

    false
}

async fn set_card_due(card_id: &str, api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting card {} due", card_id);
    Ok(())
}

async fn check_for_due_cards(key: &Key) {
    let mut card_ids: Vec<String> = Vec::new();
    match get_vocab_card_ids(&key.key).await {
        Ok(ids) => card_ids = ids,
        Err(e) => eprintln!("Error getting card ids: {}", e),
    }

    if card_ids.is_empty() {
        println!("No cards found");
        return;
    }

    for card_id in card_ids {
        match get_card_from_id(&card_id, &key.key).await {
            Ok(card) => {
                if check_if_due(card, &card_id, &key.key).await {
                    let _ = set_card_due(&card_id, &key.key).await;
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

#[tokio::main]
async fn main() {
    let key = Key::new();

    // let mut card = SimpleCard::new("Test Card", "This is a test card", None);
    // card.add_tag("api-test");

    // if let Err(e) = card.send(&key).await {
    //     eprintln!("Error sending card: {}", e);
    // }

    // check_for_due_cards(&key).await;

    println!("Uncomment code in main to run diffrent functions");
}