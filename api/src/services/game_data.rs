#![allow(dead_code)]

use crate::api::game_data_server::GameData;
use crate::api::CreateItemInstanceRequest;
use crate::api::Message;
use sonyflake::Sonyflake;
use sqlx::pool::Pool;
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
    /// TODO
    async fn get_game_server_startup_data(
        &self,
        request: Request<Message>,
    ) -> Result<Response<Message>, Status> {
        let message = format!("Request: {:?}", request);
        println!("{:?}", message);
        Ok(Response::new(Message { message }))
    }
    /// TODO
    async fn log_in_caracter(
        &self,
        request: Request<Message>,
    ) -> Result<Response<Message>, Status> {
        let message = format!("Request: {:?}", request);
        println!("{:?}", message);
        Ok(Response::new(Message { message }))
    }
    /// TODO
    async fn update_caracter(
        &self,
        request: Request<Message>,
    ) -> Result<Response<Message>, Status> {
        let message = format!("Request: {:?}", request);
        println!("{:?}", message);
        Ok(Response::new(Message { message }))
    }
    /// TODO
    async fn create_item_instance(
        &self,
        request: Request<CreateItemInstanceRequest>,
    ) -> Result<Response<Message>, Status> {
        let message = format!("Request: {:?}", request);
        println!("{:?}", message);
        Ok(Response::new(Message { message }))
    }
}
