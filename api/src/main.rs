mod api;
mod model;
mod queries;
mod services;
mod utils;
use api::{game_data_server::GameDataServer, users_server::UsersServer};
use services::game_data::GameDataService;
use services::users::UsersService;
use sqlx::SqlitePool;
use std::env;
use tonic::transport::Server;
use utils::new_sonyflake;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let database_url =
        env::var("DATABASE_URL").expect("Environment variable 'DATABASE_URL' not found.");
    let machine_id_range =
        env::var("MACHINE_ID_RANGE").expect("Environment variable 'MACHINE_ID_RANGE' not found.");
    let mut machine_ids = machine_id_range.split("..").map(|s| {
        s.parse::<u16>()
            .expect("'MACHINE_ID_RANGE' must be a pair of integers.")
    });

    let game_data_service = GameDataService::new(
        SqlitePool::connect(&database_url)
            .await
            .expect("Could not load SQLite database."),
        new_sonyflake(&mut machine_ids).unwrap(),
    );
    let users_service = UsersService::new(
        SqlitePool::connect(&database_url)
            .await
            .expect("Could not load SQLite database."),
        new_sonyflake(&mut machine_ids).unwrap(),
    );

    let addr = "[::1]:50051".parse()?;
    println!(
        "☄️ Starting Project Comet Game Data API Service on: http://{}",
        addr
    );
    Server::builder()
        .add_service(GameDataServer::new(game_data_service))
        .add_service(UsersServer::new(users_service))
        .serve(addr)
        .await?;

    Ok(())
}
