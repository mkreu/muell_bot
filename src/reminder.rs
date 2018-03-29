use chrono::prelude::*;
use chrono::Duration;
use super::dates::*;
use std::{error, fmt, thread};
use std::error::Error;
use std::sync::mpsc::*;
use std::time::Duration as StdDuration;
use std::sync::Mutex;
use tgapi::send::SendMessage;
use std::sync::Arc;
use id_list;

lazy_static! {
    static ref CUR_DATE: Mutex<Option<NaiveDate>> = Mutex::new(None);
}

#[derive(Debug)]
pub struct NoMoreDateError;

impl error::Error for NoMoreDateError {
    fn description(&self) -> &str {
        "Date Manager did not contain any future dates!"
    }
}

impl fmt::Display for NoMoreDateError {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "No Mor Date Error: {}", self.description())
     }
}

pub fn skip_current() {
    let mut handle = CUR_DATE.lock().unwrap();
    *handle = None;
}

//TODO remove this fkt
pub fn tmp_activate() {
    *CUR_DATE.lock().unwrap() = Some(NaiveDate::from_ymd(2000, 01, 01));
}

pub fn schedule_reminder(wake : DateTime<Local>, chan : Sender<SendMessage>, date_mgr : Arc<Mutex<DateMgr>>) {
    let duration = StdDuration::new((wake.timestamp() - Local::now().timestamp()) as u64, 0);
    thread::spawn(move || {
        thread::park_timeout(duration);
        remind(chan, date_mgr);
    });
}

fn remind(chan : Sender<SendMessage>, date_mgr : Arc<Mutex<DateMgr>>) {
    if let Some(_) = *CUR_DATE.lock().unwrap() {
        if let Some(next_date) = date_mgr.lock().unwrap().next_date() {
            //Send update Message
            let txt = format_update_msg(next_date.0, next_date.1);
            let subscribers = id_list::get_user_ids().unwrap();
            subscribers.iter().map(|id| SendMessage{chat_id : *id, text : txt.clone()})
                .for_each(|msg| chan.send(msg).unwrap());
            //Spawn new Thread
        }
    }
}

fn get_next_wake(active_date : &NaiveDate) -> DateTime<Local> {
    let now = Local::now();
    let mut new_wake = now;
    if &now.naive_local().date() < active_date {
        let prev_day = Local::from_local_date(active_date - Duration::days(1)).unwrap();
        if now < prev_day.and_hms(18, 0, 0) {
            prev_day.and_hms(18, 0, 0)
        } else {
            now.with_minute(0) + Duration::hours(1)
        }
    } else {
        let active_day = Local::from_local_date(active_date).unwrap();
        if now < active_day.and_hms(10, 0, 0) {
            now.with_minute(0) + Duration::hours(1)
        } else {
            active_day.and_hms(12,0,0) + Duration::days(1)
        }
    }
}

fn format_update_msg(date : &NaiveDate, trashes : Vec<&TrashType>) -> String{
    format!("Morgen ({}) kommt;\n{:?}", date, trashes)
}



/*pub fn start_reminder_loop(date_mgr : DateMgr) -> (Sender<MsgUpdate>, thread::Thread) {
    let (sender, receiver) = channel();
    (sender, thread::spawn(move|| {
        match reminder_loop(date_mgr, receiver) {
            Ok(()) => (),
            Err(NoMoreDateError) => println!("We have run out of dates, terminating"),
        }
    }).thread().to_owned())
}


fn reminder_loop(mut date_mgr : DateMgr, channel : Receiver<MsgUpdate>) -> Result<(), NoMoreDateError> {
    let mut wake = Local::now();
    let mut active_date = date_mgr.next_date().ok_or(NoMoreDateError)?.0.to_owned();
    loop {
        date_mgr.remove_old();

        //Checking if we received any updates over the channel
        wake = handle_msg(&channel, &active_date, wake);


        if wake > Local.from_local_datetime(&active_date.and_hms(9,59,0)).unwrap() {
            active_date = date_mgr.next_date().ok_or(NoMoreDateError)?.0.to_owned();
        }
        else if wake <= Local::now() {

            if wake >= Local.from_local_datetime(&(active_date.and_hms(12,0,0) - Duration::days(1))).unwrap() {

                println!("sending reminder message...");
                send_reminder(&active_date, date_mgr.next_date().ok_or(NoMoreDateError)?.1);
            }
            wake = update_wake(&active_date, wake);
        }

        //Getting new date and update to the next day
        println!("{:?}\n{:?}", &wake, &active_date);
        send_debug_message(&wake, &active_date);
        thread::park_timeout(StdDuration::new((wake.timestamp() - Local::now().timestamp()) as u64, 0));
    }
}


fn handle_msg(channel : &Receiver<MsgUpdate>, next_date : &NaiveDate, old_wake : DateTime<Local>) -> DateTime<Local>{
    let mut ret_wake = old_wake;
    for msg in channel.try_iter() {
        match msg {
            MsgUpdate::Skip => ret_wake = Local.from_local_datetime(&next_date.and_hms(10,0,0)).unwrap()
        }
    }
    ret_wake
}*/


fn send_debug_message(wake : &DateTime<Local>, active_date : &NaiveDate) {
    //TgApi::from_conf().unwrap().send(ADMIN_ID, &format!("{}\n{}", wake, active_date)).unwrap()
}

fn send_reminder(active_date : &NaiveDate, trashes : Vec<&TrashType>) {
    //TgApi::from_conf().unwrap().send(HAUS_ID, &format!("Morgen ({}) kommt;\n{:?}", active_date, trashes)).unwrap()
}



