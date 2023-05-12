use chat::Message;

pub struct ServerChat {
    pub id: i32,
    pub channel: String,
    pub messages: Vec<Message>,
}

impl ServerChat {
    pub fn new(id: i32, channel: String) -> ServerChat {
        ServerChat {
            id: id,
            channel: channel,
            messages: Vec::new(),
        }
    }
}
