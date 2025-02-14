use super::read_dir_recursive;
use crate::model::tables::{Content, GameServer, User, World};
use sqlx::{query_as, Pool, Sqlite};
use std::fs;
use toml::{map::Map, Table, Value};

pub async fn import_data(db: &Pool<Sqlite>) -> Result<(), String> {
    let mut data_toml_string = String::new();
    let files = read_dir_recursive("data").unwrap();
    print!("{:?}", files);
    for file in files {
        data_toml_string = data_toml_string + &fs::read_to_string(file.path()).unwrap();
    }
    let data = data_toml_string.parse::<Table>().unwrap();

    for (key, value) in data {
        let err_text = format!("Table definiton invalid for table: '{}'", key);

        let rows = value
            .as_array()
            .ok_or(&err_text)?
            .iter()
            .map(|value| {
                value.as_table().ok_or(format!(
                    "{}. Array does not contain key-value maps.",
                    &err_text
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;

        match key.as_str() {
            "content" => import_content_rows(db, rows)
                .await
                .map_err(|e| format!("{}. {}", &err_text, e))?,
            "user" => import_user_rows(db, rows)
                .await
                .map_err(|e| format!("{}. {}", &err_text, e))?,
            "access_token" => import_access_token_rows(db, rows)
                .await
                .map_err(|e| format!("{}. {}", &err_text, e))?,
            "game_server" => import_game_server_rows(db, rows)
                .await
                .map_err(|e| format!("{}. {}", &err_text, e))?,
            "world" => import_world_rows(db, rows)
                .await
                .map_err(|e| format!("{}. {}", &err_text, e))?,
            _ => return Err("Unsupported or non-existent table name: ".to_owned() + &key),
        };
    }
    Ok(())
}

const NO_VALUE: Value = Value::Boolean(false);

pub async fn import_content_rows(
    db: &Pool<Sqlite>,
    rows: Vec<&Map<String, Value>>,
) -> Result<(), String> {
    for row in rows {
        let data = row
            .get("data")
            .unwrap_or(&NO_VALUE)
            .as_table()
            .map(|map| serde_json::to_string(map).ok())
            .flatten();
        let new_user = query_as::<_, Content>(
            "INSERT INTO content (id, name, content_type, content_subtype, data) VALUES ($1, $2, $3, $4, $5) ON CONFLICT(id) DO UPDATE SET name=excluded.name, content_type=excluded.content_type, content_subtype=excluded.content_subtype, data=excluded.data, updated_at=(unixepoch()) RETURNING *",
        )
        .bind(row.get("id").unwrap_or(&NO_VALUE).as_integer())
        .bind(row.get("name").unwrap_or(&NO_VALUE).as_str())
        .bind(row.get("content_type").unwrap_or(&NO_VALUE).as_integer())
        .bind(row.get("content_subtype").unwrap_or(&NO_VALUE).as_integer())
        .bind(data)
        .fetch_one(db)
        .await
        .map_err(|e| e.to_string())?;
        print!("Imported Content: {:?}\n", new_user);
    }
    Ok(())
}

pub async fn import_user_rows(
    db: &Pool<Sqlite>,
    rows: Vec<&Map<String, Value>>,
) -> Result<(), String> {
    for row in rows {
        let new_user = query_as::<_, User>(
            "INSERT INTO user (id, username, role) VALUES ($1, $2, $3) ON CONFLICT(id) DO UPDATE SET username=excluded.username, role=excluded.role, updated_at=(unixepoch()) RETURNING *",
        )
        .bind(
            row.get("id")
                .unwrap_or(&NO_VALUE)
                .as_integer()
                .ok_or("Missing ID.")?,
        )
        .bind(row.get("username").unwrap_or(&NO_VALUE).as_str())
        .bind(row.get("role").unwrap_or(&NO_VALUE).as_integer())
        .fetch_one(db)
        .await
        .map_err(|e| e.to_string())?;
        print!("Imported User: {:?}\n", new_user);
    }
    Ok(())
}

pub async fn import_access_token_rows(
    _db: &Pool<Sqlite>,
    _rows: Vec<&Map<String, Value>>,
) -> Result<(), String> {
    Ok(())
}

pub async fn import_game_server_rows(
    db: &Pool<Sqlite>,
    rows: Vec<&Map<String, Value>>,
) -> Result<(), String> {
    for row in rows {
        let new_user = query_as::<_, GameServer>(
            "INSERT INTO game_server (id, region_code, display_name) VALUES ($1, $2, $3) ON CONFLICT(id) DO UPDATE SET region_code=excluded.region_code, display_name=excluded.display_name, updated_at=(unixepoch()) RETURNING *",
        )
        .bind(
            row.get("id")
                .unwrap_or(&NO_VALUE)
                .as_str()
                .ok_or("Missing ID.")?,
        )
        .bind(row.get("region_code").unwrap_or(&NO_VALUE).as_str())
        .bind(row.get("display_name").unwrap_or(&NO_VALUE).as_str())
        .fetch_one(db)
        .await
        .map_err(|e| e.to_string())?;
        print!("Imported User: {:?}\n", new_user);
    }
    Ok(())
}

pub async fn import_world_rows(
    db: &Pool<Sqlite>,
    rows: Vec<&Map<String, Value>>,
) -> Result<(), String> {
    for row in rows {
        let new_user = query_as::<_, World>(
            "INSERT INTO world (id, game_server_id, display_name) VALUES ($1, $2, $3) ON CONFLICT(id) DO UPDATE SET game_server_id=excluded.game_server_id, display_name=excluded.display_name, updated_at=(unixepoch()) RETURNING *",
        )
        .bind(
            row.get("id")
                .unwrap_or(&NO_VALUE)
                .as_str()
                .ok_or("Missing ID.")?,
        )
        .bind(row.get("game_server_id").unwrap_or(&NO_VALUE).as_str())
        .bind(row.get("display_name").unwrap_or(&NO_VALUE).as_str())
        .fetch_one(db)
        .await
        .map_err(|e| e.to_string())?;
        print!("Imported User: {:?}\n", new_user);
    }
    Ok(())
}
