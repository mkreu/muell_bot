use super::*;
use log::warn;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessage {
    pub chat_id: i64,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
}

impl SendMessage {
    pub fn txt(chat_id: i64, text: String) -> SendMessage {
        SendMessage {
            chat_id,
            text,
            parse_mode: None,
        }
    }
    pub fn md(chat_id: i64, text: String) -> SendMessage {
        SendMessage {
            chat_id,
            text,
            parse_mode: Some("markdown".to_string()),
        }
    }
}

pub fn init_send(api_conf: &ApiConf) -> mpsc::Sender<SendMessage> {
    let (tx, rx) = mpsc::channel();
    let api_string =
        String::from("https://api.telegram.org/bot") + &api_conf.token + "/sendMessage";
    thread::spawn(move || {
        let client = Client::new();
        for msg in rx.iter() {
            if let Err(e) = client.post(&api_string).json(&msg).send() {
                warn!("Error during send: {:?}", e);
            }
        }
    });
    tx
}
