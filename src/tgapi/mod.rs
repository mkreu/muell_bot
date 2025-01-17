use serde::Deserialize;
use std::fs::File;
use std::io;

pub mod receive;
pub mod send;
pub mod types;

#[derive(Deserialize)]
pub struct ApiConf {
    pub token: String,
}

pub fn read_api_conf(filename: &str) -> io::Result<ApiConf> {
    let file = File::open(filename)?;
    let conf = serde_json::from_reader(&file)?;
    Ok(conf)
}
