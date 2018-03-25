use super::*;
use reqwest::Client;
use std::sync::mpsc;
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessage {
    pub chat_id : i64,
    pub text : String
}

impl TgApi {
    pub fn init_send(&self) -> mpsc::Sender<SendMessage> {
        let (tx, rx) = mpsc::channel();
        let api_string = String::from("https://api.telegram.org/bot") + &self.api_conf.token + "/sendMessage";
        thread::spawn(move || {
            let client = Client::new();
            for msg in rx.iter() {
                if let Err(e) = client.post(&api_string).json(&msg).send() {
                    println!("Error during send: {:?}", e);
                }
            }
        });
        tx
    }
}
