use chrono::prelude::*;
use chrono::Duration;
use super::dates::*;
use std::thread;
use std::sync::mpsc::*;
use std::time::Duration as StdDuration;
use std::sync::Mutex;
use tgapi::send::SendMessage;
use std::sync::Arc;
use id_list;
use std::sync::mpsc;

/*#[derive(Debug)]
pub struct NoMoreDateError;

impl error::Error for NoMoreDateError {
    fn description(&self) -> &str {
        "Date Manager did not contain any future dates!"
    }
}

impl fmt::Display for NoMoreDateError {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "No Mor Date Error: {}", self.description());
         Ok(())
     }
}*/

struct Reminder {
    date_mgr : Arc<Mutex<DateMgr>>,
    chan : Sender<SendMessage>,
    scheduled_wakes : Vec<DateTime<Local>>,
    skip_rx : Receiver<()>
}

pub struct Skipper {
    skip_tx : Sender<()>
}

pub fn start_loop(chan : Sender<SendMessage>, date_mgr : Arc<Mutex<DateMgr>>) -> Skipper {
    let (skip_tx, skip_rx) = mpsc::channel();
    let mut reminder = Reminder{
        chan,
        date_mgr,
        scheduled_wakes : Vec::new(),
        skip_rx
    };
    thread::spawn(move || reminder.daily_update_loop());
    Skipper{skip_tx}
}

impl Skipper {
    pub fn skip(&self){
        self.skip_tx.send(()).unwrap();
    }
}

impl Reminder {

    fn fill_wakes(&mut self, next_date : &NaiveDate) {
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
        self.scheduled_wakes = to_insert.iter()
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
        if self.skip_rx.try_recv().is_ok() && self.scheduled_wakes.len() < 10{
            let last_date = self.scheduled_wakes.first().expect("wakes should never be empty when not skipped before").clone();
            println!("recived skip command");
            self.scheduled_wakes.clear();
            self.scheduled_wakes.push(last_date.clone());
        }

        // pop wake time and send reminder msg when at correct time
        if let Some(next_wake) = self.scheduled_wakes.last().map(|re| re.clone()) {
            if next_wake <= Local::now() {
                self.scheduled_wakes.pop();
                if next_wake.time() != NaiveTime::from_hms(12, 0 , 0) {
                    self.msg_update();
                }
            }
        }

        //if empty, fill with new updates
        if self.scheduled_wakes.is_empty() {
            let next_date = {
                let mut guard = self.date_mgr.lock().unwrap();
                guard.remove_old();
                guard.next_date().unwrap().0.clone()
            };
            self.fill_wakes(&next_date);
        }

        //Sleep until next wake
        if let Some(next_wake) = self.scheduled_wakes.last() {
            println!("Next wake: {:?}", &next_wake);
            sleep_until(next_wake);
        }
        else {
            panic!("wakes should never be empty")
        }
    }

    fn msg_update(&self) {
        let guard = self.date_mgr.lock().unwrap();
        let next_date = guard.next_date().unwrap();
        //Send update Message
        let txt = format_update_msg(next_date.0, next_date.1);
        let subscribers = id_list::get_user_ids().unwrap();
        subscribers.iter().map(|id| SendMessage{chat_id : *id, text : txt.clone()})
            .for_each(|msg| self.chan.send(msg).unwrap());
    }

}

fn sleep_until(time : &DateTime<Local>) {
    let dur = time.timestamp() - Local::now().timestamp();
    let i = if dur < 0 {
        1
    } else {
        dur as u64
    };
    let dur = i;
    println!("Will now sleep for {} seconds", &dur);
    thread::sleep(StdDuration::new(dur, 0))
}

fn format_update_msg(date : &NaiveDate, trashes : Vec<&TrashType>) -> String{
    let day = if date.and_hms(5, 0 ,0) > Local::now().naive_local(){
        "Morgen"
    } else {
        "Heute"
    };
    format!("{} wird der folgende Müll abgeholt:\n{}", day, format_trashes(trashes))
}

fn format_trashes(trashes : Vec<&TrashType>) -> String {
    trashes.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join("\n")
}

//Below old code
/*
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


fn handle_msg(channel : &Receiver<MsgUpdate>, next_date : &NaiveDate, old_wake : DateTime<Local>) -> DateTime<Local>{
    let mut ret_wake = old_wake;
    for msg in channel.try_iter() {
        match msg {
            MsgUpdate::Skip => ret_wake = Local.from_local_datetime(&next_date.and_hms(10,0,0)).unwrap()
        }
    }
    ret_wake
}*/


/*fn send_debug_message(wake : &DateTime<Local>, active_date : &NaiveDate) {
    //TgApi::from_conf().unwrap().send(ADMIN_ID, &format!("{}\n{}", wake, active_date)).unwrap()
}

fn send_reminder(active_date : &NaiveDate, trashes : Vec<&TrashType>) {
    //TgApi::from_conf().unwrap().send(HAUS_ID, &format!("Morgen ({}) kommt;\n{:?}", active_date, trashes)).unwrap()
}*/



