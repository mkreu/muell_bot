extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate iron;
extern crate router;

use tgapi::SendMessage;
use tgapi::types::*;
use tgapi::TgApi;

mod tgapi;


fn main() {
    //tgapi::run();
    TgApi::new().unwrap().start_listen(handle_update);
}

fn handle_update(up : Update) {
    match up.message {
        Some(m) => {
            let msg =SendMessage{chat_id : m.chat.id, text : &m.text.unwrap_or(String::from("empty message"))};
            tgapi::TgApi::new().unwrap().send(msg).unwrap()
        },
        None => println!("Empty update")
    }
}