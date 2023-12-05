use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv().ok();
    utils::core::logging::install().await;
    tracing::info!("Server starting...");
    println!("Hello, world!");
    web::server::run().await;
    Ok(())
}
