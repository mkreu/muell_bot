use super::*;
use reqwest::Client;

#[derive(Serialize, Deserialize, Debug)]
struct SendMessage<'a> {
    chat_id : i64,
    text : &'a str,
}

impl TgApi {
    pub fn send(&self, chat_id : i64, text : &str) -> Result<(), Box<Error>>{
        let api_string = String::from("https://api.telegram.org/bot") + &self.api_conf.token + "/sendMessage";
        let msg = SendMessage{chat_id, text};
        let client = Client::new();
        let _res = client.post(&api_string).json(&msg).send()?;
        Ok(())
    }
}