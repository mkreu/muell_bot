extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate iron;
extern crate router;
extern crate chrono;
extern crate reqwest;

use tgapi::types::*;
use tgapi::TgApi;
use reminder::MsgUpdate;
use dates::*;
use std::sync::mpsc::Sender;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;

mod tgapi;
mod dates;
mod reminder;

fn main() {
    //tgapi::run();
    let mut mgr = DateMgr::new();
    mgr.append_file("2018.csv").unwrap();
    mgr.remove_old();
    let api = TgApi::from_conf().unwrap();
    let (tx, thread) = reminder::start_reminder_loop(mgr);
    let rx = api.start_listen();
    let api_tx = api.init_send();
    loop {
        let update = rx.recv().unwrap();
        handle_update(update, &thread, &tx);
    }
}

fn handle_update(up : Update, thread : &thread::Thread, chan : &Sender<MsgUpdate>) {
    let api = TgApi::from_conf().unwrap();
    match up.message {
        Some(m) => {
            match m.text {
                Some(ref t) if t == "/muell" => {
                    //api.send(m.chat.id, "will be fixed in future").unwrap();
                }
                Some(ref t) if t == "/skip" => {
                    chan.send(MsgUpdate::Skip).unwrap();
                    thread.unpark();
                    //api.send(m.chat.id, &m.text.unwrap_or(String::from("empty message"))).unwrap();
                }
                _ => {
                    thread.unpark();
                    //api.send(m.chat.id, &m.text.unwrap_or(String::from("unknown command"))).unwrap();
                }
            }
        },
        None => println!("Empty update")
    }
}

fn get_next_dates(mgr : &DateMgr) -> String {
    mgr.upcoming_dates().iter()
        .map(|&(tonne, date)| format!("{}: {}", tonne.name, date.format("%Y-%m-%d")))
        .fold(String::new(), |mut string, item| {string.push_str(&item); string.push_str("\n"); string})
}

