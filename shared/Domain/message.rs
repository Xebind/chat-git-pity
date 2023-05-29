use serde::{Serialize,Deserialize};

#[derive(Deserialize, Serialize)]
pub struct MyMessage {
    pub user: String,
    pub message: String,
    pub time: String,
}

//todo: move this to shared? add more fields probably
impl MyMessage {
    fn new() -> Self {
        return MyMessage {
            user: String::from("Me"),
            message: String::from("Hello, world!"),
            time: String::from("1:00 AM"),
        };
    }

    fn new_message(message: String) -> Self {
        return MyMessage {
            user: String::from("Me"),
            message,
            time: String::from("1:00 AM"),
        };
    }
}