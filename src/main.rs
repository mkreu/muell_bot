extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate iron;
extern crate reqwest;
extern crate router;
extern crate serde_json;

use crate::dates::*;
use crate::reminder::Skipper;
use crate::tgapi::send::SendMessage;
use crate::tgapi::types::*;
use std::fs;
use std::sync::Arc;
use std::sync::Mutex;

mod dates;
mod id_list;
mod reminder;
mod tgapi;

fn main() {
    let mut mgr = DateMgr::new();
    for path in fs::read_dir("dates").unwrap() {
        mgr.append_file(path.unwrap().path()).unwrap();
    }
    mgr.remove_old();
    let mgr = Arc::new(Mutex::new(mgr));
    let api = tgapi::read_api_conf("API.conf").unwrap();
    let api_rx = tgapi::receive::start_listen(&api);
    let api_tx = tgapi::send::init_send(&api);
    let reminder = reminder::start_loop(api_tx.clone(), mgr.clone());
    loop {
        let update = api_rx.recv().unwrap();
        if let Some(msg) = handle_update(update, &mgr, &reminder) {
            api_tx.send(msg).unwrap();
        }
    }
}

fn handle_update(up: Update, mgr: &Arc<Mutex<DateMgr>>, reminder: &Skipper) -> Option<SendMessage> {
    match up.message {
        Some(m) => match m.text {
            Some(ref t) if t.starts_with("/muell") => {
                let dates = mgr.lock().unwrap();
                let text = get_next_dates(&*dates);
                Some(SendMessage::md(m.chat.id, text))
            }
            Some(ref t) if t.starts_with("/skip") => {
                reminder.skip();
                Some(SendMessage::md(
                    m.chat.id,
                    String::from(
                        "_Der Müll wurde also rausgebracht._ 🎉\nDann bin ich jetzt ruhig",
                    ),
                ))
            }
            Some(ref t) if t.starts_with("/start") => {
                id_list::add_user(m.chat.id).unwrap();
                Some(SendMessage::txt(m.chat.id, String::from("Welcome to the Müllbot! You have joined the notification list. /stop to leave")))
            }
            Some(ref t) if t.starts_with("/stop") => {
                id_list::remove_user(m.chat.id).unwrap();
                Some(SendMessage::txt(
                    m.chat.id,
                    String::from("Removed you from notification list. Type /start to rejoin"),
                ))
            }
            _ => Some(SendMessage::txt(m.chat.id, String::from("unknown command"))),
        },
        None => {
            println!("Empty update");
            None
        }
    }
}

fn get_next_dates(mgr: &DateMgr) -> String {
    mgr.upcoming_dates()
        .iter()
        .map(|&(tonne, date)| format!("*{}:*\n    `{}`", tonne, date.format("%Y-%m-%d")))
        .fold(String::new(), |mut string, item| {
            string.push_str(&item);
            string.push('\n');
            string
        })
}
