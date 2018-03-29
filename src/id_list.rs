use serde_json;
use std::io;
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;

const CLIENT_FILE : &'static str = "Clients.json";

#[derive(Serialize, Deserialize)]
struct IdList {
    admins : HashSet<i64>,
    subscribers : HashSet<i64>
}

fn read_list() -> io::Result<IdList> {
    let list = if Path::new(CLIENT_FILE).is_file() {
        let file = File::open(CLIENT_FILE)?;
        serde_json::from_reader(&file)?
    } else {
        IdList{admins: HashSet::new(), subscribers : HashSet::new()}
    };
    Ok(list)
}

fn write_list(list : IdList) -> io::Result<()> {
    serde_json::to_writer(File::create(CLIENT_FILE)?, &list);
    Ok(())
}

pub fn get_user_ids() -> io::Result<HashSet<i64>> {
    Ok(read_list()?.subscribers)
}

pub fn get_admin_ids() -> io::Result<HashSet<i64>> {
    Ok(read_list()?.admins)
}

pub fn add_user(id : i64) -> io::Result<()> {
    let mut list = read_list()?;
    list.subscribers.insert(id);
    write_list(list)?;
    Ok(())
}

pub fn add_admin(id : i64) -> io::Result<()> {
    let mut list = read_list()?;
    list.admins.insert(id);
    write_list(list)?;
    Ok(())
}
