#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id: i32,
    pub message: Option<Message>,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    //message_id : i32
    pub chat: Chat,
    pub text: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Chat {
    pub id: i64, //type : String
}
