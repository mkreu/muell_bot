use chrono::prelude::*;
use chrono::Duration;
use super::dates::*;
use std::{error, fmt, thread};
use std::error::Error;
use std::sync::mpsc::*;
use std::time::Duration as StdDuration;

static ADMIN_ID : i64 = 176074613;
static HAUS_ID : i64 = -211192143; //TODO

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

pub enum MsgUpdate {
    Skip,
    //Poll
}

pub fn start_reminder_loop(date_mgr : DateMgr) -> (Sender<MsgUpdate>, thread::Thread) {
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

fn update_wake(active_date : &NaiveDate, old_wake : DateTime<Local>) -> DateTime<Local> {
    let mut new_wake = old_wake;
    if &old_wake.naive_local().date() != active_date {
        new_wake = Local.from_local_datetime(&(active_date.and_hms(12, 0, 0) - Duration::days(1))).unwrap();
    }
    else if old_wake.time() == NaiveTime::from_hms(12,0,0) {
        new_wake = old_wake + Duration::hours(6);
    }
    while new_wake < Local::now() {
        new_wake = old_wake + Duration::hours(2);
    }
    new_wake
}

fn handle_msg(channel : &Receiver<MsgUpdate>, next_date : &NaiveDate, old_wake : DateTime<Local>) -> DateTime<Local>{
    let mut ret_wake = old_wake;
    for msg in channel.try_iter() {
        match msg {
            MsgUpdate::Skip => ret_wake = Local.from_local_datetime(&next_date.and_hms(10,0,0)).unwrap()
        }
    }
    ret_wake
}


fn send_debug_message(wake : &DateTime<Local>, active_date : &NaiveDate) {
    //TgApi::from_conf().unwrap().send(ADMIN_ID, &format!("{}\n{}", wake, active_date)).unwrap()
}

fn send_reminder(active_date : &NaiveDate, trashes : Vec<&TrashType>) {
    //TgApi::from_conf().unwrap().send(HAUS_ID, &format!("Morgen ({}) kommt;\n{:?}", active_date, trashes)).unwrap()
}



