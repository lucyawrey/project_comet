use crate::game_data_api;
use crate::utils::next_id;
use game_data_api::character_server::Character;
use game_data_api::create_character_request::{HomeWorld, Player};
use game_data_api::{CreateCharacterRequest, MessageReply};
use sonyflake::Sonyflake;
use sqlx::Pool;
use sqlx::Sqlite;
use tonic::{Request, Response, Status};

pub struct CharacterService {
    db: Pool<Sqlite>,
    sf: Sonyflake,
}

impl CharacterService {
    pub fn new(db: Pool<Sqlite>, sf: Sonyflake) -> CharacterService {
        CharacterService { db, sf }
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

        let (id, created_at) = next_id(&self.sf)?;
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
            message: format!(
                "Created character: '{}' with ID '{}` at time: `{}`",
                args.name, new_id, created_at
            ), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };
        Ok(Response::new(reply))
    }
}
