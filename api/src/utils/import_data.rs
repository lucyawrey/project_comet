use sqlx::{query_as, Pool, Sqlite};
use std::fs;
use toml::{Table, Value};

use crate::model::tables::{Content, User};

pub async fn import_data(db: &Pool<Sqlite>) -> Result<(), String> {
    let mut data_toml_string = String::new();
    let directory = fs::read_dir("data").unwrap();
    for file in directory {
        data_toml_string = data_toml_string + &fs::read_to_string(file.unwrap().path()).unwrap();
    }
    let data = data_toml_string.parse::<Table>().unwrap();
    for (key, value) in data {
        let err_text = format!("Table definiton invalid for table with name: {}", key);
        let rows = value.as_array().ok_or(&err_text)?;
        match key.as_str() {
            "content" => import_content_rows(db, rows).await.map_err(|_e| err_text)?,
            "user" => import_user_rows(db, rows).await.map_err(|_e| err_text)?,
            "access_token" => import_access_token_rows(db, rows)
                .await
                .map_err(|_e| err_text)?,
            "game_server" => import_game_server_rows(db, rows)
                .await
                .map_err(|_e| err_text)?,
            "world" => import_world_rows(db, rows).await.map_err(|_e| err_text)?,
            _ => return Err("Unsupported or non-existent table name: ".to_owned() + &key),
        };
    }
    Ok(())
}

pub async fn import_content_rows(db: &Pool<Sqlite>, rows: &Vec<Value>) -> Result<(), ()> {
    for value in rows {
        let row = value.as_table().ok_or(())?;
        let data = serde_json::to_string(row.get("data").ok_or(())?).map_err(|_e| ())?;
        print!("Data: {:?}", data);
        let new_user = query_as::<_, Content>(
            "INSERT INTO content (id, name, content_type, content_subtype, data) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(row.get("id").ok_or(())?.as_integer().ok_or(())?)
        .bind(row.get("name").ok_or(())?.as_str().ok_or(())?)
        .bind(row.get("content_type").ok_or(())?.as_integer().ok_or(())?)
        .bind(row.get("content_subtype").ok_or(())?.as_integer().ok_or(())?)
        .bind(data)
        .fetch_one(db)
        .await
        .unwrap();
        print!("New User: {:?}", new_user)
    }
    Ok(())
}

pub async fn import_user_rows(db: &Pool<Sqlite>, rows: &Vec<Value>) -> Result<(), ()> {
    for value in rows {
        let row = value.as_table().ok_or(())?;
        let new_user = query_as::<_, User>(
            "INSERT INTO user (id, username, role) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(row.get("id").ok_or(())?.as_integer().ok_or(())?)
        .bind(row.get("username").ok_or(())?.as_str().ok_or(())?)
        .bind(row.get("role").ok_or(())?.as_integer().ok_or(())?)
        .fetch_one(db)
        .await
        .map_err(|_e| ())?;
        print!("New User: {:?}", new_user)
    }
    Ok(())
}

pub async fn import_access_token_rows(db: &Pool<Sqlite>, rows: &Vec<Value>) -> Result<(), ()> {
    Ok(())
}

pub async fn import_game_server_rows(db: &Pool<Sqlite>, rows: &Vec<Value>) -> Result<(), ()> {
    Ok(())
}

pub async fn import_world_rows(db: &Pool<Sqlite>, rows: &Vec<Value>) -> Result<(), ()> {
    Ok(())
}
