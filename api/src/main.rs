mod character_service;
mod database;
mod game_data_api;
mod utils;
use character_service::CharacterService;
use game_data_api::character_server::CharacterServer;
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
    let sf = Sonyflake::new().expect("Could not setup snowflake ID generator.");

    let addr = "[::1]:50051".parse()?;
    println!(
        "☄️ Starting Project Comet Game Data API Service on: http://{}",
        addr
    );
    let character = CharacterService::new(db, sf);
    Server::builder()
        .add_service(CharacterServer::new(character))
        .serve(addr)
        .await?;

    Ok(())
}
