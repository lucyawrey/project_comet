use crate::{
    api::{
        create_character_request::UserRef, users_server::Users, Character, CreateCharacterRequest,
        Message,
    },
    model::{fields::Role, tables::User},
    utils::{generate_random_name, next_id, validate_and_format_name},
};
use sonyflake::Sonyflake;
use sqlx::pool::Pool;
use sqlx::Sqlite;
use tonic::{Request, Response, Status};

pub struct UsersService {
    db: Pool<Sqlite>,
    sf: Sonyflake,
}

impl UsersService {
    pub fn new(db: Pool<Sqlite>, sf: Sonyflake) -> UsersService {
        UsersService { db, sf }
    }
}

#[tonic::async_trait]
impl Users for UsersService {
    async fn create_character(
        &self,
        request: Request<CreateCharacterRequest>,
    ) -> Result<Response<Character>, Status> {
        println!("  Got a request: {:?}", request);
        let args = request.into_inner();

        let name = match args.name {
            Some(name) => validate_and_format_name(name)
                .ok_or(Status::internal("Character name is invalid."))?,
            None => generate_random_name(),
        };
        let home_world_id = args.home_world_id;

        let user = match args
            .user_ref
            .ok_or(Status::internal("Must provide user ID or username."))?
        {
            UserRef::UserUsername(username) => {
                sqlx::query!("SELECT * FROM user WHERE username = $1", username)
                    .try_map(|r| {
                        Ok(User {
                            id: r.id,
                            updated_at: r.updated_at,
                            username: r.username,
                            role: Role::try_from(r.role as i32).unwrap(),
                        })
                    })
                    .fetch_one(&self.db)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?
            }
            UserRef::UserId(id) => sqlx::query!("SELECT * FROM user WHERE id = $1", id)
                .try_map(|r| {
                    Ok(User {
                        id: r.id,
                        updated_at: r.updated_at,
                        username: r.username,
                        role: Role::try_from(r.role as i32).unwrap(),
                    })
                })
                .fetch_one(&self.db)
                .await
                .map_err(|e| Status::internal(e.to_string()))?,
        };
        let role = match args.role {
            Some(role) => {
                if role > user.role.into() {
                    return Err(Status::internal(
                        "Character role cannot have higher access level than user role.",
                    ));
                }
                Role::try_from(role).unwrap()
            }
            None => user.role,
        };
        let role_int: i32 = role.into();

        let (id, created_at, _) = next_id(&self.sf)?;
        let new = sqlx::query!(
            "INSERT INTO character (id, updated_at, name, role, home_world_id, user_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            id,
            created_at,
            name,
            role_int,
            home_world_id,
            user.id,
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Character {
            id: new.id,
            updated_at: new.updated_at,
            name: new.name,
            role: new.role as i32,
            home_world_id: new.home_world_id,
            user_id: new.user_id,
            ancestry: new.ancestry as i32,
            gender: new.gender as i32,
            customization: new.customization,
            data: new.data,
        }))
    }
    /// TODO
    async fn sign_up(&self, request: Request<Message>) -> Result<Response<Message>, Status> {
        let message = format!("Request: {:?}", request);
        println!("{:?}", message);
        Ok(Response::new(Message { message }))
    }
    /// TODO
    async fn log_in(&self, request: Request<Message>) -> Result<Response<Message>, Status> {
        let message = format!("Request: {:?}", request);
        println!("{:?}", message);
        Ok(Response::new(Message { message }))
    }
    /// TODO
    async fn log_out(&self, request: Request<Message>) -> Result<Response<Message>, Status> {
        let message = format!("Request: {:?}", request);
        println!("{:?}", message);
        Ok(Response::new(Message { message }))
    }
    /// TODO
    async fn delete_user(&self, request: Request<Message>) -> Result<Response<Message>, Status> {
        let message = format!("Request: {:?}", request);
        println!("{:?}", message);
        Ok(Response::new(Message { message }))
    }
    /// TODO
    async fn update_username(
        &self,
        request: Request<Message>,
    ) -> Result<Response<Message>, Status> {
        let message = format!("Request: {:?}", request);
        println!("{:?}", message);
        Ok(Response::new(Message { message }))
    }
    /// TODO
    async fn update_password(
        &self,
        request: Request<Message>,
    ) -> Result<Response<Message>, Status> {
        let message = format!("Request: {:?}", request);
        println!("{:?}", message);
        Ok(Response::new(Message { message }))
    }
    /// TODO
    async fn verify_recovery_code(
        &self,
        request: Request<Message>,
    ) -> Result<Response<Message>, Status> {
        let message = format!("Request: {:?}", request);
        println!("{:?}", message);
        Ok(Response::new(Message { message }))
    }
    /// TODO
    async fn recover_user(&self, request: Request<Message>) -> Result<Response<Message>, Status> {
        let message = format!("Request: {:?}", request);
        println!("{:?}", message);
        Ok(Response::new(Message { message }))
    }
    /// TODO
    async fn get_character(&self, request: Request<Message>) -> Result<Response<Message>, Status> {
        let message = format!("Request: {:?}", request);
        println!("{:?}", message);
        Ok(Response::new(Message { message }))
    }
    /// TODO
    async fn get_characters(&self, request: Request<Message>) -> Result<Response<Message>, Status> {
        let message = format!("Request: {:?}", request);
        println!("{:?}", message);
        Ok(Response::new(Message { message }))
    }
    /// TODO
    async fn delete_character(
        &self,
        request: Request<Message>,
    ) -> Result<Response<Message>, Status> {
        let message = format!("Request: {:?}", request);
        println!("{:?}", message);
        Ok(Response::new(Message { message }))
    }
}
