use game_data::character_server::{Character, CharacterServer};
use game_data::{CreateCharacterReply, CreateCharacterRequest};
use sqlx::SqlitePool;
use sqlx::{migrate::MigrateDatabase, Sqlite};
use tonic::{transport::Server, Request, Response, Status};

pub mod game_data {
    tonic::include_proto!("game_data");
}

#[derive(Debug, Default)]
pub struct CharacterService {}

#[tonic::async_trait]
impl Character for CharacterService {
    async fn create(
        &self,
        request: Request<CreateCharacterRequest>, // Accept request of type CreateCharacterRequest
    ) -> Result<Response<CreateCharacterReply>, Status> {
        // Return an instance of type CreateCharacterRequest
        println!("  Got a request: {:?}", request);

        let reply = CreateCharacterReply {
            message: format!("Created character: {}.", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };
        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

const DATABASE_URL: &str = "sqlite://game_data.sqlite";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    if !Sqlite::database_exists(DATABASE_URL).await.unwrap_or(false) {
        println!("  Creating database {}.", DATABASE_URL);
        match Sqlite::create_database(DATABASE_URL).await {
            Ok(_) => println!("  Create database success."),
            Err(error) => panic!("  error: {}", error),
        }
    } else {
        println!("  Loading existing database.");
    }
    let db = SqlitePool::connect(DATABASE_URL).await.unwrap();
    println!("  Running database migrations.");
    sqlx::migrate!().run(&db).await.unwrap();

    let addr = "[::1]:50051".parse()?;
    println!("☄️ Starting Project Comet Game Data API Service on: {}", addr);
    let greeter = CharacterService::default();
    Server::builder()
        .add_service(CharacterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
