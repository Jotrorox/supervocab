use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Serialize;
use serde_json::Value;
use std::{collections::HashMap, thread, time};

mod simple_card;
use simple_card::SimpleCard;

mod constants;
use constants::BASE_URL;

mod key;
use key::Key;

mod tag_updater;
use tag_updater::*;

mod markup_updater;
use markup_updater::*;

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

async fn get_card_from_id(
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

async fn check_if_due(mut resp: Value, card_id: &str, api_key: &str) -> bool {
    println!("checking for due cards");
    if resp[card_id]["membership"]["status"] != 2 {
        return false;
    }

    if &resp[card_id]["data"]["tags"]
        .clone()
        .to_owned()
        .as_array_mut()
        .unwrap()
        .len()
        <= &1
    {
        let mut card_updates = CardTagUpdates::new();
        card_updates.insert(
            card_id.to_string(),
            CardTagUpdate {
                data: CardTagData {
                    tags: vec![
                        "super-vocab-stage-0".to_string(),
                        "super-vocab-card".to_string(),
                    ],
                },
            },
        );
        let mut tag_updater = TagUpdater::new(vec![card_id.to_string()], card_updates);
        match tag_updater.update_tags().await {
            Ok(tu_resp) => resp = tu_resp,
            Err(e) => eprintln!("Error updating tags: {}", e),
        }
        return true;
    }
    let mut given_time_string = format!("{}Z", resp[card_id]["data"]["modified_when"]);
    given_time_string.retain(|c| c != '\"');
    let given_time = DateTime::parse_from_rfc3339(&given_time_string)
        .unwrap()
        .with_timezone(&Utc);
    let current_time = Utc::now();
    let duration = current_time - given_time;
    let days_difference = duration.num_days();

    let tags = resp[card_id]["data"]["tags"].as_array().unwrap();

    if tags.contains(&serde_json::Value::String(
        "super-vocab-stage-0".to_string(),
    )) {
        return true;
    } else if tags.contains(&serde_json::Value::String(
        "super-vocab-stage-1".to_string(),
    )) {
        if days_difference >= 1 {
            return true;
        }
    } else if tags.contains(&serde_json::Value::String(
        "super-vocab-stage-2".to_string(),
    )) {
        if days_difference >= 3 {
            return true;
        } else {
            return false;
        }
    } else if tags.contains(&serde_json::Value::String(
        "super-vocab-stage-3".to_string(),
    )) {
        if days_difference >= 7 {
            return true;
        } else {
            return false;
        }
    } else if tags.contains(&serde_json::Value::String(
        "super-vocab-stage-4".to_string(),
    )) {
        if days_difference >= 14 {
            return true;
        } else {
            return false;
        }
    } else if tags.contains(&serde_json::Value::String(
        "super-vocab-stage-5".to_string(),
    )) {
        if days_difference >= 30 {
            return true;
        } else {
            return false;
        }
    } else if tags.contains(&serde_json::Value::String(
        "super-vocab-stage-6".to_string(),
    )) {
        if days_difference >= 90 {
            return true;
        } else {
            return false;
        }
    } else if tags.contains(&serde_json::Value::String(
        "super-vocab-stage-7".to_string(),
    )) {
        if days_difference >= 180 {
            return true;
        } else {
            return false;
        }
    } else if tags.contains(&serde_json::Value::String(
        "super-vocab-stage-8".to_string(),
    )) {
        if days_difference >= 365 {
            return true;
        } else {
            return false;
        }
    }

    false
}

async fn set_card_due(card_id: &str, api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting cards due");
    let resp: Value = get_card_from_id(card_id, api_key).await?;

    let mut updated_card_markup = resp[card_id]["data"]["markup"]
        .clone()
        .to_string()
        .replace("[X]", "[ ]");

    updated_card_markup.retain(|c| c != '\"');

    let mut card_updates = CardMarkupUpdates::new();
    card_updates.insert(
        card_id.to_string(),
        CardMarkupUpdate {
            data: CardMarkupData {
                markup: updated_card_markup,
            },
        },
    );
    let mut markup_updater = MarkupUpdater::new(vec![card_id.to_string()], card_updates);
    markup_updater.update_markup().await.unwrap();

    let mut current_state: String = values_to_strings(resp[card_id]["data"]["tags"].as_array())
        .iter()
        .filter(|&element| element.contains("super-vocab-stage-"))
        .map(|s| s.as_str())
        .collect();

    current_state = current_state.replace("\"", "");

    if current_state == "super-vocab-stage-0" {
        current_state = "super-vocab-stage-1".to_string();
    } else if current_state == "super-vocab-stage-1" {
        current_state = "super-vocab-stage-2".to_string();
    } else if current_state == "super-vocab-stage-2" {
        current_state = "super-vocab-stage-3".to_string();
    } else if current_state == "super-vocab-stage-3" {
        current_state = "super-vocab-stage-4".to_string();
    } else if current_state == "super-vocab-stage-4" {
        current_state = "super-vocab-stage-5".to_string();
    } else if current_state == "super-vocab-stage-5" {
        current_state = "super-vocab-stage-6".to_string();
    } else if current_state == "super-vocab-stage-6" {
        current_state = "super-vocab-stage-7".to_string();
    } else if current_state == "super-vocab-stage-7" {
        current_state = "super-vocab-stage-8".to_string();
    } else {
        current_state = "super-vocab-stage-8".to_string();
    }

    let mut card_updates = CardTagUpdates::new();
    card_updates.insert(
        card_id.to_string(),
        CardTagUpdate {
            data: CardTagData {
                tags: vec![current_state, "super-vocab-card".to_string()],
            },
        },
    );
    let mut tag_updater = TagUpdater::new(vec![card_id.to_string()], card_updates);
    tag_updater.update_tags().await.unwrap();

    Ok(())
}

async fn check_for_due_cards(key: &Key) {
    let mut card_ids: Vec<String> = Vec::new();
    match get_vocab_card_ids(&key.key).await {
        Ok(ids) => card_ids = ids,
        Err(e) => eprintln!("Error getting card ids: {}", e),
    }

    if card_ids.is_empty() {
        return;
    }

    for card_id in card_ids {
        match get_card_from_id(&card_id, &key.key).await {
            Ok(card) => {
                if check_if_due(card, &card_id, &key.key).await {
                    let _ = set_card_due(&card_id, &key.key).await;
                } else {
                    println!("{}", card_id);
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn values_to_strings(option_values: Option<&Vec<Value>>) -> Vec<String> {
    option_values
        .map(|values| {
            values
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<String>>()
        })
        .unwrap_or_else(|| Vec::new())
}

#[tokio::main]
async fn main() {
    let key = Key::new();

    // let mut card = SimpleCard::new("Test Card", "This is a test card", None);
    // card.add_tag("api-test");

    // if let Err(e) = card.send(&key).await {
    //     eprintln!("Error sending card: {}", e);
    // }

    loop {
        check_for_due_cards(&key).await;
        thread::sleep(time::Duration::from_secs(60 * /* Minutes: */ 15))
    }
}
