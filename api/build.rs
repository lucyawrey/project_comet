use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let database_url =
        env::var("DATABASE_URL").expect("Environment variable DATABASE_URL not found.");
    tokio::executor::block_on(migrate(&database_url));

    tonic_build::compile_protos("proto/game_data.proto")?;
    Ok(())
}

async fn migrate(database_url: &str) {
    if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
        println!("  Creating database {}.", database_url);
        match Sqlite::create_database(database_url).await {
            Ok(_) => println!("  Create database success."),
            Err(error) => panic!("  error: {}", error),
        }
    } else {
        println!("  Loading existing database.");
    }
    let db = SqlitePool::connect(database_url).await.unwrap();
    println!("  Running database migrations.");
    sqlx::migrate!().run(&db).await.unwrap();
}
