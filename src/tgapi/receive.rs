use iron::prelude::*;
use iron::status;
use router::Router;
use super::types::Update;
use serde_json;
use super::TgApi;
use std::thread;
use std::sync::mpsc;
use std::sync::Mutex;

impl TgApi {
    pub fn start_listen(&self) -> mpsc::Receiver<Update> {
        let mut router = Router::new();
        let (tx, rx) = mpsc::channel();
        let mutex = Mutex::new(tx);
        router.post(&self.api_conf.webhook_path, move |req: &mut Request| webhook_handle(req,  &mutex), "tgapi");
        let addr = self.api_conf.webhook_addr;
        thread::spawn(move||Iron::new(router).http(&addr).unwrap());
        rx
    }
}

fn webhook_handle(req : &mut Request, chan : &Mutex<mpsc::Sender<Update>>) -> IronResult<Response> {
    println!("recieved webhook request");
    match serde_json::from_reader(&mut req.body) {
        Ok(u) => {
            chan.lock().unwrap().send(u).unwrap();
            Ok(Response::with(status::Ok))
        },
        Err(_) => {
            println!("could not parse json!");
            Ok(Response::with((status::BadRequest)))
        },
    }
}

