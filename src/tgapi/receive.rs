use super::types::Update;
use crate::tgapi::ApiConf;
use log::warn;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Deserialize)]
struct ApiResponse {
    result: Vec<Update>,
}

pub fn start_listen(api_conf: &ApiConf) -> mpsc::Receiver<Update> {
    let client = Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .expect("failed to build recieve client");
    let (tx, rx) = mpsc::channel();
    let api_string = format!("https://api.telegram.org/bot{}/getUpdates", &api_conf.token);
    let mut offset = 0;
    thread::spawn(move || loop {
        let req_string = format!("{}?offset={}&timeout=100", api_string, offset);
        match client
            .get(&req_string)
            .send()
            .and_then(|req| req.json::<ApiResponse>())
        {
            Err(e) => warn!("Error getting Updates: {:?}", e),
            Ok(resp) => {
                offset = resp
                    .result
                    .iter()
                    .map(|up| up.update_id + 1)
                    .max()
                    .unwrap_or(offset);
                resp.result.into_iter().for_each(|up| tx.send(up).unwrap());
            }
        }
    });
    rx
}
