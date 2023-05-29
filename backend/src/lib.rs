use std::{
    collections::{HashSet, VecDeque},
    net::SocketAddr,
    sync::{Arc, Mutex},

};
use axum::{response::{Html, IntoResponse, Redirect}, routing::{get, post, MethodFilter}, Router, extract::State, Json};
use axum::extract::path::ErrorKind::Message;
use axum::http::StatusCode;
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[path="../../shared/Domain/message.rs"]
pub mod message;
use crate::message::MyMessage;

struct AppState {
    user_set: Mutex<HashSet<String>>,
    tx: broadcast::Sender<String>,
    message_history : Mutex<VecDeque<MyMessage>>
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>>  {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_chat=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let user_set = Mutex::new(HashSet::new());
    //todo: change magic numbers for constants
    let (tx, _) = broadcast::channel(100);
    let  message_history: Mutex<VecDeque<MyMessage>> =  Mutex::new(VecDeque::with_capacity(25));

    let app_sate = Arc::new(AppState { user_set, tx, message_history });

    let app = Router::new()
        .route("/", get(get_messages))
        .route("/health_check", get(health_check))
        .route("/sendMessage", post(add_message))
        .layer(CorsLayer::permissive())

        //.route("/getMessages", get(get_messages))
        .with_state(app_sate);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())

}

async fn health_check() -> StatusCode {
    StatusCode::OK
}


async fn add_message(State(AppState): State<Arc<AppState>>, Json(message): Json<MyMessage>) -> Redirect
{
    if AppState.message_history.lock().unwrap().len() >= 25 {
        AppState.message_history.lock().unwrap().pop_front();
    }
    AppState.message_history.lock().unwrap().push_back(message);

    //todo: broadcast to all clients

    return Redirect::to("/");

}

async fn get_messages(State(AppState): State<Arc<AppState>>) -> String
{
    let mut string = String::new();
    for message in AppState.message_history.lock().unwrap().iter(){
        string.push_str(&format!("{:?} \n", message.message));

    }
    return string
}