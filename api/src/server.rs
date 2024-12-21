use game_data::character_server::{Character, CharacterServer};
use game_data::create_character_request::{HomeWorld, Player};
use game_data::{CreateCharacterRequest, MessageReply};
use sonyflake::Sonyflake;
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

pub struct CharacterService {
    db: Pool<Sqlite>,
    sf: Sonyflake,
}
impl CharacterService {
    fn new(db: Pool<Sqlite>, sf: Sonyflake) -> CharacterService {
        CharacterService { db, sf }
    }
}

pub fn next_id(sf: &Sonyflake) -> Result<i64, Status> {
    match sf.next_id() {
        Ok(id) => Ok(id as i64),
        Err(e) => Err(Status::internal(e.to_string())),
    }
}

#[tonic::async_trait]
impl Character for CharacterService {
    async fn create(
        &self,
        request: Request<CreateCharacterRequest>, // Accept request of type CreateCharacterRequest
    ) -> Result<Response<MessageReply>, Status> {
        // Return an instance of type CreateCharacterRequest
        println!("  Got a request: {:?}", request);
        let args = request.into_inner();
        let home_world_id = match args
            .home_world
            .ok_or(Status::internal("Must provide a world ID or name."))?
        {
            HomeWorld::HomeWorldName(name) => {
                sqlx::query!("SELECT (id) FROM world WHERE name = $1", name)
                    .fetch_one(&self.db)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?
                    .id
            }
            HomeWorld::HomeWorldId(id) => id,
        };
        let player_id = match args
            .player
            .ok_or(Status::internal("Must provide player ID or username."))?
        {
            Player::PlayerUsername(username) => {
                sqlx::query!("SELECT (id) FROM player WHERE username = $1", username)
                    .fetch_one(&self.db)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?
                    .id
            }
            Player::PlayerId(id) => id,
        };

        let id = next_id(&self.sf)?;
        let new_id = sqlx::query!(
            "INSERT INTO character (id, name, home_world_id, player_id) VALUES ($1, $2, $3, $4)",
            id,
            args.name,
            home_world_id,
            player_id,
        )
        .execute(&self.db)
        .await
        .map_err(|e| Status::internal(e.to_string()))?
        .last_insert_rowid();

        let reply = MessageReply {
            message: format!("Created character: '{}' with ID '{}'", args.name, new_id), // We must use .into_inner() as the fields of gRPC requests and responses are private
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
