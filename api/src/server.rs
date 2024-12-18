use game_data::character_server::{Character, CharacterServer};
use game_data::{CreateCharacterReply, CreateCharacterRequest};
use sqlx::Sqlite;
use sqlx::{Pool, SqlitePool};
use std::env;
use tonic::{transport::Server, Request, Response, Status};

pub mod game_data {
    tonic::include_proto!("game_data");
}

pub enum PlayerRole {
    Equipped = 0,
    Inventory = 1,
    Box = 2,
}

pub enum ItemLocation {
    Equipped = 0,
    Inventory = 1,
    Box = 2,
}

pub enum ItemType {
    Currency = 0,
    Material = 1,
    Consumable = 2,
    QuestItem = 3,
    UnlockItem = 4,
    Equipment = 5,
}

#[derive(Debug)]
pub struct CharacterService {
    db: Pool<Sqlite>,
}
impl CharacterService {
    fn new(db: Pool<Sqlite>) -> CharacterService {
        CharacterService { db }
    }
}

#[tonic::async_trait]
impl Character for CharacterService {
    async fn create(
        &self,
        request: Request<CreateCharacterRequest>, // Accept request of type CreateCharacterRequest
    ) -> Result<Response<CreateCharacterReply>, Status> {
        // Return an instance of type CreateCharacterRequest
        println!("  Got a request: {:?}", request);
        let name = request.into_inner().name;

        let id = sqlx::query!(
            "INSERT INTO character (name, player_id) VALUES ($1, $2)",
            name,
            1
        )
        .execute(&self.db)
        .await
        .map_err(|e| Status::from_error(e.into()))?
        .last_insert_rowid();

        let reply = CreateCharacterReply {
            message: format!("Created character: {} with ID {}.", name, id), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    dotenvy::dotenv()?;

    let database_url =
        env::var("DATABASE_URL").expect("Environment variable DATABASE_URL not found.");
    let db = SqlitePool::connect(&database_url)
        .await
        .expect("Could not load SQLite database.");

    let addr = "[::1]:50051".parse()?;
    println!(
        "☄️ Starting Project Comet Game Data API Service on: http://{}",
        addr
    );
    let character = CharacterService::new(db);
    Server::builder()
        .add_service(CharacterServer::new(character))
        .serve(addr)
        .await?;

    Ok(())
}
