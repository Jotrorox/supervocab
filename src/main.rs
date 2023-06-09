use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;
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
                arg: "super-vocaber-card",
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
    specified: String,
}

async fn get_card_from_id(card_id: &str, api_key: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let url = format!("{}/cards/get/specify", BASE_URL);
    let card_request = CardRequest {
        specified: card_id.to_string(),
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

    // Handle the response as needed
    println!("Request Debug at line 143: {:?}", response);

    let json = response.json::<serde_json::Value>().await?;

    Ok(json)
}

async fn check_if_due(resp: Value, api_key: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // Do request using id
    Ok(false)
}

async fn set_card_due(card_id: &str, api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Do request using id
    Ok(())
}

async fn check_for_due_cards(key: &Key) {
    let mut card_ids: Vec<String> = Vec::new();
    match get_vocab_card_ids(&key.key).await {
        Ok(ids) => card_ids = ids,
        Err(e) => eprintln!("Error getting card ids: {}", e),
    }

    if card_ids.is_empty() {
        println!("No due cards found");
        return;
    }

    for card_id in card_ids {
        match get_card_from_id(&card_id, &key.key).await {
            Ok(card) => {
                match check_if_due(card, &key.key).await {
                    Ok(due) => {
                        if due {
                            match set_card_due(&card_id, &key.key).await {
                                Ok(_) => println!("Card {} set to due", &card_id),
                                Err(e) => println!("Error setting card due: {}", e),
                            }
                        }
                    },
                    Err(e) => println!("Error checking if card is due: {}", e),
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
