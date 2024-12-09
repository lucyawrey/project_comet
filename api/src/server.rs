use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use sqlx::SqlitePool;
use sqlx::{migrate::MigrateDatabase, Sqlite};
use tonic::{transport::Server, Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("hello_world");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> {
        // Return an instance of type HelloReply
        println!("  Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
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
    let greeter = MyGreeter::default();
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
