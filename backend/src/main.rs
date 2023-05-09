//use tonic::{transport::Server, Request, Response, Status};
//
//use chat::chat_server::{Chat, ChatServer};
//use chat::Message;
//
//pub mod chat {
//    tonic::include_proto!("chat");
//}
//
//#[derive(Debug, Default)]
//pub struct MyChat {}
//
//#[tonic::async_trait]
//impl Chat for MyChat {
//    async fn send_message(&self, request: Request<Message>) -> Result<Response<Message>, Status> {
//        println!("Got a request: {:?}", request);
//
//        let reply = chat::Message {
//            username: "Server".to_string(),
//            text: format!("Pong!"),
//        };
//
//        Ok(Response::new(reply))
//    }
//}

use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tokio::sync::broadcast;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

struct AppState {
    user_set: Mutex<HashSet<String>>,
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let addr = "0.0.0.0:9090".parse().unwrap();

    //    let chat = MyChat::default();
    //
    //    println!("Server running. Listening on {}", addr);
    //
    //    Server::builder()
    //        .add_service(ChatServer::new(chat))
    //        .serve(addr)
    //        .await?;
    //
    //    Ok(())

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_chat=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let user_set = Mutex::new(HashSet::new());
    let (tx, _) = broadcast::channel(100);

    let app_sate = Arc::new(AppState { user_set, tx });

    let app = Router::new().route("/", get(index)).with_state(app_sate);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())

}

async fn index() -> Html<&'static str> {
    Html("Hello, World!")
}
