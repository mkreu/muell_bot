extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate iron;
extern crate router;
extern crate chrono;

use tgapi::SendMessage;
use tgapi::types::*;
use tgapi::TgApi;
use dates::*;

mod tgapi;
mod dates;


fn main() {
    //tgapi::run();
    let mut mgr = DateMgr::new();
    mgr.append_file("termine.csv").unwrap();
    TgApi::new().unwrap().start_listen(move |u| handle_update(u, &mgr));
}

fn handle_update(up : Update, mgr : &DateMgr) {
    match up.message {
        Some(m) => {
            match m.text {
                Some(ref t) if t == "/next" => {
                    let msg =SendMessage{chat_id : m.chat.id, text : &get_next_dates(mgr)};
                    tgapi::TgApi::new().unwrap().send(msg).unwrap()
                }
                _ => {
                    let msg =SendMessage{chat_id : m.chat.id, text : &m.text.unwrap_or(String::from("empty message"))};
                    tgapi::TgApi::new().unwrap().send(msg).unwrap()
                }
            }
        },
        None => println!("Empty update")
    }
}

fn get_next_dates(mgr : &DateMgr) -> String {
    mgr.dates().iter()
        .filter(|entry| entry.1.get(0).is_some())
        .map(|(tonne, date)| format!("{} : {}", tonne.name, date.get(0).unwrap().format("%Y-%m-%d")))
        .fold(String::new(), |mut string, item| {string.push_str(&item); string.push_str("\n"); string})
}