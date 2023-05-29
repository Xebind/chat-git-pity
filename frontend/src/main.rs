use yew::prelude::*;
use reqwest;
use web_sys::{HtmlInputElement};
use wasm_bindgen::{JsCast, JsValue};
use gloo_console::log;
use serde_json::json;
use wasm_bindgen_futures;


#[path="../../shared/Domain/message.rs"]
pub mod message;
use crate::message::MyMessage;


#[function_component(App)]
fn app() -> Html {
    html! {
          <div class="container">
              <div class="app">
              <div class="chat-messages">
              <div class="chat">
                <div class="chat-content clearfix">
                 </div>

                <div class="msg-box">
                <input
                    type="text"
                    class="ip-msg"
                    placeholder="type something.."
                    id = "input"
                />
                  <span class="btn-group">
                    <button onclick={|_| {
                        let input = web_sys::window()
                                    .unwrap()
                                    .document()
                                    .unwrap()
                                    .get_element_by_id("input")
                                    .unwrap()
                                    .dyn_into::<web_sys::HtmlInputElement>()
                                    .unwrap();
                        wasm_bindgen_futures::spawn_local(async move{
                            send_message(&input.value()).await;
                        });
                    }}>
                        {"Send"}
                    </button>
                    </span>
                </div>

              </div>
            </div>
         </div>
    </div>
      }
}
async fn send_message(message: &str) {
    log!(message);

    let my_message = json!({
        "user": "me",
        "message": message,
        "time": "1:00 AM"
    });

    let client = reqwest::Client::new();
        log!("client instanciated");

    let res = client.post("http://localhost:8080/sendMessage")
        .json(&my_message)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
        log!("response arrived?");
    log!(res)
}

async fn get_messages()  {
    let client = reqwest::Client::new();
        log!("client instanciated");

    let res = client.get("http://localhost:8080/")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
        log!("response arrived?");
    log!(res)
}

fn main() {
    yew::Renderer::<App>::new().render();
}

