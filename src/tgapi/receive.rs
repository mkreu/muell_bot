use super::types::Update;
use iron::prelude::*;
use iron::status;
use router::Router;
use serde_json;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;
use tgapi::ApiConf;

pub fn start_listen(api_conf: &ApiConf) -> mpsc::Receiver<Update> {
    let mut router = Router::new();
    let (tx, rx) = mpsc::channel();
    let mutex = Mutex::new(tx);
    router.post(
        &api_conf.webhook_path,
        move |req: &mut Request| webhook_handle(req, &mutex),
        "tgapi",
    );
    let addr = api_conf.webhook_addr;
    let mut iron = Iron::new(router);
    iron.threads = 4;
    thread::spawn(move || iron.http(&addr).unwrap());
    rx
}

fn webhook_handle(req: &mut Request, chan: &Mutex<mpsc::Sender<Update>>) -> IronResult<Response> {
    println!("recieved webhook request");
    match serde_json::from_reader(&mut req.body) {
        Ok(u) => {
            chan.lock().unwrap().send(u).unwrap();
            Ok(Response::with(status::Ok))
        }
        Err(_) => {
            println!("could not parse json!");
            Ok(Response::with(status::BadRequest))
        }
    }
}
