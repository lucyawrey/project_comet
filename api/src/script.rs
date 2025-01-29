use chrono::DateTime;
use sonyflake::{decompose, Builder};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::{env, process};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(s) => match s.as_str() {
            "migrate" => {
                migrate().await?;
            }
            "id_gen" => {
                let id_count: u32 = args.get(2).unwrap_or(&"1".to_owned()).parse().unwrap_or(1);
                id_gen(id_count).await?;
            }
            _ => {
                println!(
                    "Provided script name is not valid. Valid options are 'migrate' and 'id_gen'."
                );
                process::exit(1);
            }
        },
        None => {
            println!("Please provide a valid script name as the first argument. Valid options are 'migrate' and 'id_gen'.");
            process::exit(1);
        }
    };
    Ok(())
}

async fn migrate() -> Result<(), Box<dyn std::error::Error>> {
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

async fn id_gen(id_count: u32) -> Result<(), Box<dyn std::error::Error>> {
    let sf = Builder::new()
        .start_time(DateTime::UNIX_EPOCH)
        .machine_id(&|| Ok(env::var("MACHINE_ID")?.parse::<u16>()?))
        .finalize()
        .expect("Failed to initialize ID generator");
    println!("id,timestamp,machine_id");
    for _ in 0..id_count {
        let line = match sf.next_id() {
            Ok(id) => {
                let decomposed_id = decompose(id);
                format!("{},{},{}", id, decomposed_id.time, decomposed_id.machine_id)
            }
            Err(_) => "error failed to generate id,,".to_owned(),
        };
        println!("{}", line);
    }
    Ok(())
}
