use chatgipity_backend::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    run().await
}