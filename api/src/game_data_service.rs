use crate::game_data_api;
use crate::utils::{next_id, validate_and_format_name};
use game_data_api::create_character_request::{HomeWorld, Player};
use game_data_api::game_data_server::GameData;
use game_data_api::{CreateCharacterRequest, CreateItemInstanceRequest, MessageReply};
use sonyflake::Sonyflake;
use sqlx::Pool;
use sqlx::Sqlite;
use tonic::{Request, Response, Status};

pub struct GameDataService {
    db: Pool<Sqlite>,
    sf: Sonyflake,
}

impl GameDataService {
    pub fn new(db: Pool<Sqlite>, sf: Sonyflake) -> GameDataService {
        GameDataService { db, sf }
    }
}

#[tonic::async_trait]
impl GameData for GameDataService {
    async fn create_caracter(
        &self,
        request: Request<CreateCharacterRequest>, // Accept request of type CreateCharacterRequest
    ) -> Result<Response<MessageReply>, Status> {
        // Return an instance of type CreateCharacterRequest
        println!("  Got a request: {:?}", request);
        let args = request.into_inner();
        let name = validate_and_format_name(args.name)
            .ok_or(Status::internal("CHaracter name is invalid."))?;
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

        let (id, created_at, machine_id) = next_id(&self.sf)?;
        let new_id = sqlx::query!(
            "INSERT INTO character (id, updated_at, name, home_world_id, player_id) VALUES ($1, $2, $3, $4, $5)",
            id,
            created_at,
            name,
            home_world_id,
            player_id,
        )
        .execute(&self.db)
        .await
        .map_err(|e| Status::internal(e.to_string()))?
        .last_insert_rowid();

        let reply = MessageReply {
            message: format!(
                "created character. name: '{}', id: '{}`, time: `{}`, machine_id: {}",
                name, new_id, created_at, machine_id
            ), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };
        Ok(Response::new(reply))
    }

    async fn create_item_instance(
        &self,
        request: Request<CreateItemInstanceRequest>,
    ) -> Result<Response<MessageReply>, Status> {
        println!("  Got a request: {:?}", request);
        let reply = MessageReply {
            message: format!("Request: {:?}", request),
        };
        Ok(Response::new(reply))
    }
}
