use sqlx::{migrate::MigrateDatabase, Sqlite};
use sqlx::SqlitePool;

const DATABASE_URL: &str = "sqlite://game_db.sqlite";

#[tokio::main]
async fn main() {
    println!("☄️ Starting Project Comet Game Data API Service.");
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
    sqlx::migrate!()
        .run(&db)
        .await.unwrap();
}