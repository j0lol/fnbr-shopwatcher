extern crate core;

use std::{fs, time};
use std::thread::sleep;
use feed_rs::model::Entry;
use feed_rs::parser;

use serde_derive::{Deserialize, Serialize};
use toml::Value;
use toml::value::Array;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Config {
	webhook: String,
    items: Vec<String>,
}


struct WebhookEmbed {
    content: String,
    username: String,
    pfp: String,
}

fn main() {
	let shop_contents = fetch("https://fortnite-api.com/v2/shop/br".to_string());
	let config = load_config();
	
	for i in config.items {
		if shop_contents.contains(i.as_str()) {
			hook(config.webhook.as_str(),
				WebhookEmbed {
                    content: i.clone(), username: "Gunnar the shop watcher".to_string(),
                    pfp: "https://static.wikia.nocookie.net/fortnite/images/e/e9/Gunnar_-_Outfit_-_Fortnite.png/revision/latest?cb=20220320124920".to_string()
                })
		}
	}
}

fn load_config() -> (Config) {
    let config = match fs::read_to_string("../shopwatcher.toml") {
        Ok(value) => value,
        Err(_) => unimplemented!(),
    };
    toml::from_str(config.as_str() ).unwrap()
}

fn fetch(url: String) -> String {
    let body: String = ureq::get(url.as_str())
        .call().unwrap()
        .into_string().unwrap();

    body
}

fn hook(url: &str, webhook: WebhookEmbed) {
    ureq::post(url)
        .send_json(ureq::json!({
            "avatar_url": webhook.pfp,
            "username": webhook.username,
            "content": format!("Item {} is in the store!", webhook.content)
      })).unwrap()
        .into_string().unwrap();
}
