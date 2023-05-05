use yew::prelude::*;


struct Message {
    user: String,
    message: String,
    time: String,
}

impl Message {
    fn new() -> Self {
       return Message { user: String::from("Me"), message: String::from("Hello, world!"), time: String::from("1:00 AM")};
    }

    fn new_message(message: String) -> Self {
       return Message { user: String::from("Me"), message, time: String::from("1:00 AM")};
    }
}

#[function_component(App)]
fn app() -> Html {

    html! {
        <div class="container">
            <div class="app">
            <div class="chat-messages">
            <div class="chat">
              <div class="chat-content clearfix">
                <span class="friend last">
                      {"Hi, How are You?"}
                      <span class="time">
                        {"7:30 PM"}
                      </span>
                </span>
                <span class="you first">
                      {"Hi, I am fine.
                      How about you?"}
                      <span class="time">
                        {"7:31 PM"}
                      </span>
                </span>
                <span class="you last">
                      {"lets meet,
                      this sunday!"}
                      <span class="time">
                        {"7:31 PM"}
                      </span>
                </span>
              </div>

              <div class="msg-box">
                <input type="text" class="ip-msg" placeholder="type something.."/>
                <span class="btn-group">
                      <button class="send"> {"Send"}</button>
                    </span>
              </div>

            </div>
          </div>
       </div>
  </div>
    }
}
fn get_messages() -> Vec<Message>{
    let messages: Vec<Message> = Vec::new();
    //todo get last 25 messages from ddbb?

    return messages;
}

fn main() {
    yew::Renderer::<App>::new().render();
}