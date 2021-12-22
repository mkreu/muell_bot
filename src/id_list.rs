use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::path::Path;

const CLIENT_FILE: &str = "Clients.json";

#[derive(Serialize, Deserialize)]
struct IdList {
    subscribers: HashSet<i64>,
}

fn read_list() -> io::Result<IdList> {
    let list = if Path::new(CLIENT_FILE).is_file() {
        let file = File::open(CLIENT_FILE)?;
        serde_json::from_reader(&file)?
    } else {
        IdList {
            subscribers: HashSet::new(),
        }
    };
    Ok(list)
}

fn write_list(list: IdList) -> io::Result<()> {
    serde_json::to_writer(File::create(CLIENT_FILE)?, &list)?;
    Ok(())
}

pub fn get_user_ids() -> io::Result<HashSet<i64>> {
    Ok(read_list()?.subscribers)
}

pub fn add_user(id: i64) -> io::Result<()> {
    let mut list = read_list()?;
    list.subscribers.insert(id);
    write_list(list)?;
    Ok(())
}

pub fn remove_user(id: i64) -> io::Result<()> {
    let mut list = read_list()?;
    list.subscribers.remove(&id);
    write_list(list)?;
    Ok(())
}
