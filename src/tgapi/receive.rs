use iron::prelude::*;
use iron::status;
use router::Router;
use super::types::Update;
use std::marker::*;
use serde_json;
use super::TgApi;
use std::thread;

impl TgApi {
    pub fn start_listen<F>(&self, callback: F) where F: Fn(Update) + Send + Sync + 'static {
        let mut router = Router::new();
        router.post(&self.api_conf.webhook_path, move |req: &mut Request| webhook_handle(req, &callback), "tgapi");

        let addr = self.api_conf.webhook_addr.clone();
        thread::spawn(move||Iron::new(router).http(&addr).unwrap());
    }
}

fn webhook_handle<F>(req : &mut Request, callback : &F) -> IronResult<Response> where F: Fn(Update) {
    println!("recieved webhook request");
    match serde_json::from_reader(&mut req.body) {
        Ok(u) => {
            callback(u);
            Ok(Response::with(status::Ok))
        },
        Err(_) => {
            println!("could not parse json!");
            Ok(Response::with((status::BadRequest)))
        },
    }
}

