use serde_json;
use std::net::SocketAddr;
use std::fs::File;
use std::io;

pub mod receive;
pub mod types;
pub mod send;


#[derive(Deserialize)]
pub struct ApiConf {
    pub token : String,
    pub webhook_addr : SocketAddr,
    pub webhook_path : String
}

pub fn read_api_conf(filename : &str) -> io::Result<ApiConf> {
    let file = File::open(filename)?;
    let conf = serde_json::from_reader(&file)?;
    Ok(conf)
}