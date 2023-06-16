use crate::{
    card_getter::{get_card_from_id, get_vocab_card_ids},
    key::Key,
    markup_updater::MarkupUpdater,
    values_to_strings, CardMarkupData, CardMarkupUpdate, CardMarkupUpdates, CardTagData,
    CardTagUpdate, CardTagUpdates, TagUpdater,
};
use chrono::{DateTime, Utc};
use serde_json::Value;

pub async fn check_if_due(resp: Value, card_id: &str) -> bool {
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
            Ok(_) => return true,
            Err(e) => eprintln!("Error updating tags: {}", e),
        }
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

pub async fn set_card_due(card_id: &str, api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting cards due");
    let resp: Value = get_card_from_id(card_id, api_key).await?;

    // DEBUG
    println!("response: {}", resp.to_string());

    let mut updated_card_markup = resp[card_id]["data"]["markup"]
        .clone()
        .to_string()
        .replace("[X]", "[ ]");

    updated_card_markup.retain(|c| c != '\"');
    updated_card_markup = updated_card_markup.replace("\\n", "\n");

    // DEBUG
    println!("card_markup: {}", resp[card_id]["data"]["markup"].clone().to_string());
    println!("card_markup_update: {}", updated_card_markup);

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

pub async fn check_for_due_cards(key: &Key) {
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
                if check_if_due(card, &card_id).await {
                    let _ = set_card_due(&card_id, &key.key).await;
                } else {
                    println!("{}", card_id);
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
