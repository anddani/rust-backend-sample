
#[tokio::main]
async fn main() -> Result<(), ()> {
    tracing::info!("Server starting...");
    println!("Hello, world!");
    web::server::run().await;
    Ok(())
}
