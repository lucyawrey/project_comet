use crate::{
    api::{
        create_character_request::UserRef, users_server::Users, Character, CreateCharacterRequest,
        Message,
    },
    model::{fields::Role, Ref},
    queries::character::create_character_query,
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

        let new = create_character_query(
            &self.db,
            &self.sf,
            match args
                .user_ref
                .ok_or(Status::internal("Must provide user ID or Username."))?
            {
                UserRef::Id(id) => Ref::Id(id),
                UserRef::Username(name) => Ref::Name(name),
            },
            args.home_world_id,
            args.name,
            args.role.map(|s| Role::try_from(s).unwrap()),
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Character {
            id: new.id,
            updated_at: new.updated_at.and_utc().timestamp(),
            name: new.name,
            role: new.role,
            home_world_id: new.home_world_id,
            user_id: new.user_id,
            ancestry: new.ancestry,
            gender: new.gender,
            customization: "TODO: Serialize Json<Customization>".to_owned(),
            data: "TODO: Serialize Json<CharacterData>".to_owned(),
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
