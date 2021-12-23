use super::dates::*;
use crate::id_list;
use crate::tgapi::send::SendMessage;
use chrono::prelude::*;
use chrono::Duration;
use log::info;
use std::sync::mpsc;
use std::sync::mpsc::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration as StdDuration;

struct Reminder {
    date_mgr: Arc<Mutex<DateMgr>>,
    chan: Sender<SendMessage>,
    scheduled_wakes: Vec<DateTime<Local>>,
    skip_rx: Receiver<()>,
}

pub struct Skipper {
    skip_tx: Sender<()>,
}

pub fn start_loop(chan: Sender<SendMessage>, date_mgr: Arc<Mutex<DateMgr>>) -> Skipper {
    let (skip_tx, skip_rx) = mpsc::channel();
    let mut reminder = Reminder {
        chan,
        date_mgr,
        scheduled_wakes: Vec::new(),
        skip_rx,
    };
    thread::spawn(move || reminder.daily_update_loop());
    Skipper { skip_tx }
}

impl Skipper {
    pub fn skip(&self) {
        self.skip_tx.send(()).unwrap();
    }
}

impl Reminder {
    fn fill_wakes(&mut self, next_date: &NaiveDate) {
        let before_date = *next_date - Duration::days(1);
        let mut to_insert = vec![
            before_date.and_hms(13, 0, 0),
            before_date.and_hms(18, 0, 0),
            before_date.and_hms(20, 0, 0),
            before_date.and_hms(22, 0, 0),
            next_date.and_hms(0, 0, 0),
            next_date.and_hms(2, 0, 0),
            next_date.and_hms(4, 0, 0),
            next_date.and_hms(6, 0, 0),
            next_date.and_hms(8, 0, 0),
            next_date.and_hms(12, 0, 0),
        ];
        to_insert.reverse();
        self.scheduled_wakes = to_insert
            .iter()
            .map(|date| Local.from_local_datetime(date).unwrap())
            .collect();
    }
    fn daily_update_loop(&mut self) {
        loop {
            self.reminder_update();
        }
    }

    fn reminder_update(&mut self) {
        // remove all but 12 o clock reminder if skip was called
        if self.skip_rx.try_recv().is_ok() && self.scheduled_wakes.len() < 10 {
            let last_date = *self
                .scheduled_wakes
                .first()
                .expect("wakes should never be empty when not skipped before");
            info!("recived skip command");
            self.scheduled_wakes.clear();
            self.scheduled_wakes.push(last_date);
        }
        //cleaning up skip queue
        while self.skip_rx.try_recv().is_ok() {}

        // pop wake time and send reminder msg when at correct time
        if let Some(next_wake) = self.scheduled_wakes.last().copied() {
            if next_wake <= Local::now() {
                self.scheduled_wakes.pop();
                if next_wake.time() != NaiveTime::from_hms(12, 0, 0) {
                    self.msg_update();
                }
            }
        }

        //if empty, fill with new updates
        if self.scheduled_wakes.is_empty() {
            let next_date = {
                let mut guard = self.date_mgr.lock().unwrap();
                guard.remove_old();
                *guard.next_date().unwrap().0
            };
            self.fill_wakes(&next_date);
        }

        //Sleep until next wake
        if let Some(next_wake) = self.scheduled_wakes.last() {
            info!("Next wake: {:?}", &next_wake);
            sleep_until(next_wake);
        } else {
            panic!("wakes should never be empty")
        }
    }

    fn msg_update(&self) {
        let guard = self.date_mgr.lock().unwrap();
        let next_date = guard.next_date().unwrap();
        //Send update Message
        let txt = format_update_msg(next_date.0, next_date.1);
        let subscribers = id_list::get_user_ids().unwrap();
        subscribers
            .iter()
            .map(|id| SendMessage::md(*id, txt.clone()))
            .for_each(|msg| self.chan.send(msg).unwrap());
    }
}

fn sleep_until(time: &DateTime<Local>) {
    let dur = time.timestamp() - Local::now().timestamp();
    let i = if dur < 0 { 1 } else { dur as u64 };
    let dur = i;
    info!("Will now sleep for {} seconds", &dur);
    thread::sleep(StdDuration::new(dur, 0))
}

fn format_update_msg(date: &NaiveDate, trashes: Vec<&TrashType>) -> String {
    let day = if date.and_hms(5, 0, 0) > Local::now().naive_local() {
        "Morgen"
    } else {
        "Heute"
    };
    format!(
        "_{} wird der folgende Müll abgeholt:_\n{}",
        day,
        format_trashes(trashes)
    )
}

fn format_trashes(trashes: Vec<&TrashType>) -> String {
    trashes
        .iter()
        .map(|t| format!("*{}*", t))
        .collect::<Vec<String>>()
        .join("\n")
}
