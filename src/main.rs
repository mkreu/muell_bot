extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate iron;
extern crate router;
extern crate chrono;
extern crate reqwest;

use tgapi::types::*;
use reminder::MsgUpdate;
use dates::*;
use std::sync::mpsc::Sender;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;
use std::sync::Arc;
use tgapi::send::SendMessage;

mod tgapi;
mod dates;
mod reminder;
mod id_list;

fn main() {
    //tgapi::run();
    let mut mgr = DateMgr::new();
    mgr.append_file("2018.csv").unwrap();
    mgr.remove_old();
    let mgr = Arc::new(Mutex::new(mgr));
    let api = tgapi::read_api_conf("API.conf").unwrap();
    //let (tx, thread) = reminder::start_reminder_loop(mgr);
    let api_rx = tgapi::receive::start_listen(&api);
    let api_tx = tgapi::send::init_send(&api);
    loop {
        let update = api_rx.recv().unwrap();
        if let Some(msg) = handle_update(update,&mgr) {
            api_tx.send(msg);
        }
    }
}


fn handle_update(up : Update, mgr : &Arc<Mutex<DateMgr>>) -> Option<SendMessage> {
    match up.message {
        Some(m) => {
            match m.text {
                Some(ref t) if t == "/muell" => {
                    let mut dates = mgr.lock().unwrap();
                    let text = get_next_dates(&*dates);
                    Some(SendMessage {chat_id : m.chat.id, text})
                }
                Some(ref t) if t == "/skip" => {
                    Some(SendMessage{chat_id : m.chat.id, text : String::from("skipping")})
                }
                _ => {
                    Some(SendMessage{chat_id : m.chat.id, text : String::from("unknown command")})
                }
            }
        },
        None =>  {
            println!("Empty update");
            None
        }
    }
}

fn get_next_dates(mgr : &DateMgr) -> String {
    mgr.upcoming_dates().iter()
        .map(|&(tonne, date)| format!("{}: {}", tonne.name, date.format("%Y-%m-%d")))
        .fold(String::new(), |mut string, item| {string.push_str(&item); string.push_str("\n"); string})
}

