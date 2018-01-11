use serde_json;
use std::error::Error;
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

pub struct TgApi {
    api_conf : ApiConf
}
impl TgApi {
    pub fn from_conf() -> io::Result<TgApi> {
        let api_conf = read_api_conf("API.conf")?;
        Ok(TgApi {
            api_conf
        })
    }

    /*pub fn send(&self, message : SendMessage) -> Result<(), Box<Error>>{
        let api_string = String::from("https://api.telegram.org/bot") + &self.api_conf.token + "/sendMessage";
        println!("{}", &api_string);

        let json_msg = serde_json::to_string(&message)?;
        println!("in JSON: {}", json_msg);
        let mut core = Core::new()?;
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle)?)
            .build(&handle);
        let uri = api_string.parse()?;
        let mut req = Request::new(Method::Post, uri);
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(json_msg.len() as u64));
        req.set_body(json_msg);
        let post = client.request(req).and_then(|res| {
            println!("POST: {}", res.status());
            res.body().concat2()
        });
        core.run(post)?;
        Ok(())
    }*/
}

fn read_api_conf(filename : &str) -> io::Result<ApiConf> {
    let file = File::open(filename)?;
    let conf = serde_json::from_reader(&file)?;
    Ok(conf)
}