use tonic::{transport::Server, Request, Response, Status};

use chat::chat_server::{Chat, ChatServer};
use chat::Message;

pub mod chat {
    tonic::include_proto!("chat");
}

#[derive(Debug, Default)]
pub struct MyChat {}

#[tonic::async_trait]
impl Chat for MyChat {
    async fn send_message(&self, request: Request<Message>) -> Result<Response<Message>, Status> {
        println!("Got a request: {:?}", request);

        let reply = chat::Message {
            username: "Server".to_string(),
            text: format!("Pong!"),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let chat = MyChat::default();

    Server::builder()
        .add_service(ChatServer::new(chat))
        .serve(addr)
        .await?;

    Ok(())
}
