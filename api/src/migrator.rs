use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let database_url =
        env::var("DATABASE_URL").expect("Environment variable DATABASE_URL not found.");

    if !Sqlite::database_exists(&database_url)
        .await
        .unwrap_or(false)
    {
        println!("  Creating database {}.", &database_url);
        match Sqlite::create_database(&database_url).await {
            Ok(_) => println!("  Create database success."),
            Err(error) => panic!("  error: {}", error),
        }
    } else {
        println!("  Loading existing database.");
    }
    let db = SqlitePool::connect(&database_url).await?;
    println!("  Running database migrations.");
    sqlx::migrate!().run(&db).await?;

    Ok(())
}
