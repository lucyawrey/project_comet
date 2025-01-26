mod database;
mod game_data_api;
mod game_data_service;
mod utils;
use game_data_api::game_data_server::GameDataServer;
use game_data_service::GameDataService;
use sonyflake::Sonyflake;
use sqlx::SqlitePool;
use std::env;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    dotenvy::dotenv()?;

    let database_url =
        env::var("DATABASE_URL").expect("Environment variable DATABASE_URL not found.");
    let db = SqlitePool::connect(&database_url)
        .await
        .expect("Could not load SQLite database.");
    // TODO customize snowflake ID generation.
    let sf = Sonyflake::new().expect("Could not setup the snowflake ID generator.");

    let addr = "[::1]:50051".parse()?;
    println!(
        "☄️ Starting Project Comet Game Data API Service on: http://{}",
        addr
    );
    let service = GameDataService::new(db, sf);
    Server::builder()
        .add_service(GameDataServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
