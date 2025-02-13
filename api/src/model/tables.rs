#![allow(dead_code)]

use super::fields::{
    AccessLevel, CharacterData, CharacterStatusData, ClassData, CompanionCollectionEntryData,
    ContentData, ContentSubtype, ContentType, Customization, GameOptionsData, GameOptionsType,
    GuildRole, ItemCollectionEntryLocation, ItemInstanceData, ItemInstanceLocation,
    ItemInstanceQuality, Statistics,
};
use sqlx::types::Json;

/* User Service Schema */
#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i64,          // Snowflake ID, alias of rowid
    pub updated_at: i64,  // Unix timestamp with 10 msec precision
    pub username: String, // Unique no case
    // pub role: Role,
    pub role: i32,
}

#[derive(sqlx::FromRow)]
pub struct UserPassword {
    pub id: i64,      // Snowflake ID, alias of rowid
    pub user_id: i64, // Snowflake ID, referances a `User`
    pub password_hash: String,
}

#[derive(sqlx::FromRow)]
pub struct UserSession {
    pub id: String,      // Hash of the generated user session token
    pub expires_at: i64, // Unix timestamp with 10 msec precision a certain time in the future
    pub user_id: i64,    // Snowflake ID, referances a `User`
}

#[derive(sqlx::FromRow)]
pub struct UserRecoveryCode {
    pub id: String,   // Hash of the generated user account recovery code
    pub user_id: i64, // Snowflake ID, referances a `User`
    pub temporary: bool,
}
/* End User Service Schema */

/* Administration Service Schema */
#[derive(sqlx::FromRow)]
pub struct AccessToken {
    id: i64,                   // Snowflake ID, alias of rowid
    access_token_hash: String, // Hash of the generated access token. Token format is: `default|server:gameserverid|admin_IdBase64Representation_secret`
    access_level: AccessLevel,
    game_server_id: String,  // String ID, referances a 'GameServer'
    expires_at: Option<i64>, // Unix timestamp with 10 msec precision a certain time in the future}
}

#[derive(sqlx::FromRow)]
pub struct GameServer {
    pub id: String, // Case insensitive String ID, should be input in lowercase with no spaces
    pub created_at: i64, // Unix timestamp with 10 msec precision
    pub updated_at: i64, // Unix timestamp with 10 msec precision
    pub region_code: String, // Server location represented by a timezone, using case sensitive tz database identifiers. Ex: 'US/Eastern'
    pub display_name: String, // Server name for end user display
}

#[derive(sqlx::FromRow)]
pub struct World {
    pub id: String, // Case insensitive String ID, should be input in lowercase with no spaces
    pub created_at: i64, // Unix timestamp with 10 msec precision
    pub updated_at: i64, // Unix timestamp with 10 msec precision
    pub game_server_id: String, // String ID, referances a 'GameServer'
    pub display_name: String, // Server name for end user display
}
/* End Administration Service Schema */

/* Game Data Service Schema */
#[derive(sqlx::FromRow)]
pub struct Character {
    pub id: i64,         // Snowflake ID, alias of rowid
    pub updated_at: i64, // Unix timestamp with 10 msec precision
    pub name: String, // Unique no case with `home_world_id`. Character is initially created with a silly random name.
    // pub role: Role, // Same type as `User.role`, `Character.role` can be a lower rank than `User.role` but should never be higher than it.
    pub role: i32,
    pub home_world_id: String, // String ID, referances a 'World'
    pub user_id: i64,          // Snowflake ID, referances a `User`
    // pub ancestry: CharacterAncestry,
    pub ancestry: i32,
    // pub gender: CharacterGender,
    pub gender: i32,
    pub customization: Json<Customization>,
    pub data: Json<CharacterData>,
}

#[derive(sqlx::FromRow)]
pub struct GameOptions {
    pub id: i64,         // Snowflake ID, alias of rowid
    pub updated_at: i64, // Unix timestamp with 10 msec precision
    pub game_options_type: GameOptionsType,
    pub data: Json<GameOptionsData>,
    pub user_id: Option<i64>,      // Snowflake ID, referances a `User`
    pub character_id: Option<i64>, // Snowflake ID, referances a `Character`
}

#[derive(sqlx::FromRow)]
pub struct CharacterStatus {
    pub id: i64,              // Snowflake ID, alias of rowid
    pub updated_at: i64,      // Unix timestamp with 10 msec precision
    pub character_id: i64,    // Snowflake ID, referances a `Character`
    pub active_class_id: i64, // Snowflake ID, referances a `Class`
    pub statistics: Json<Statistics>,
    pub data: Json<CharacterStatusData>,
    pub base_gearset_id: i64, // Snowflake ID, referances a `Gearset`
    pub base_outfit_id: i64,  // Snowflake ID, referances an `Item`
    pub active_gearset_id: Option<i64>, // Snowflake ID, referances a `Gearset`
    pub active_outfit_id: Option<i64>, // Snowflake ID, referances an `Item`
    pub active_class_item_id: Option<i64>, // Snowflake ID, referances an `Item`
}

#[derive(sqlx::FromRow)]
pub struct Class {
    pub id: i64,               // Snowflake ID, alias of rowid
    pub updated_at: i64,       // Unix timestamp with 10 msec precision
    pub character_id: i64,     // Snowflake ID, referances a `Character`
    pub class_content_id: i64, // Snowflake ID, referances a `Content`
    pub exsperience: i64,
    pub level: i64,
    pub is_unlocked: bool,
    pub statistics: Json<Statistics>,
    pub data: Json<ClassData>,
    pub class_item_id: Option<i64>, // Snowflake ID, referances an `Item`
}

#[derive(sqlx::FromRow)]
pub struct Gearset {
    pub id: i64,           // Snowflake ID, alias of rowid
    pub updated_at: i64,   // Unix timestamp with 10 msec precision
    pub character_id: i64, // Snowflake ID, referances a `Character`
    pub name: String, // Case insensitive indexed name, special value BASE means this is the default gearset that is directly modified when equipping gear
    pub statistics: Json<Statistics>,
    pub linked_class_id: Option<i64>, // Snowflake ID, referances a `Class`
    pub linked_outfit_id: Option<i64>, // Snowflake ID, referances a `Outfit`
    pub item_id: [Option<i64>; 16],   // Snowflake ID array, referances multiple `Item`s
}

#[derive(sqlx::FromRow)]
pub struct Outfit {
    pub id: i64,           // Snowflake ID, alias of rowid
    pub updated_at: i64,   // Unix timestamp with 10 msec precision
    pub character_id: i64, // Snowflake ID, referances a `Character`
    pub name: String, // Case insensitive indexed name, special value BASE means this is the default gearset that is directly modified when equipping gear
    pub customization: Json<Customization>,
    pub linked_class_id: Option<i64>, // Snowflake ID, referances a `Class`
    pub linked_outfit_id: Option<i64>, // Snowflake ID, referances a `Outfit`
    pub item_content_id: [Option<i64>; 16], // Snowflake ID array, referances multiple `Content`s
}

#[derive(sqlx::FromRow)]
pub struct Friendship {
    pub id: i64,             // Snowflake ID, alias of rowid
    pub character_id_0: i64, // Snowflake ID, referances a `Character`
    pub character_id_1: i64, // Snowflake ID, referances a `Character`
}

#[derive(sqlx::FromRow)]
pub struct Guild {
    pub id: i64,               // Snowflake ID, alias of rowid
    pub updated_at: i64,       // Unix timestamp with 10 msec precision
    pub name: String,          // Unique no case
    pub home_world_id: String, // String ID, referances a 'World'
}

#[derive(sqlx::FromRow)]
pub struct GuildMembership {
    pub id: i64,           // Snowflake ID, alias of rowid
    pub guild_id: i64,     // Snowflake ID, referances a `Guild`
    pub character_id: i64, // Snowflake ID, referances a `Character`
    pub role: GuildRole,
}

#[derive(sqlx::FromRow)]
pub struct Item {
    pub id: i64,              // Snowflake ID, alias of rowid
    pub character_id: i64,    // Snowflake ID, referances a `Character`
    pub item_content_id: i64, // Snowflake ID, referances an `Item`
    pub quantity: i64, // Quantitiy can only be above item's `stack_size` when in a box. `is_unique` items never stack. Items can only stack if they have the same `location`, `quality`, `craft_character_id` and no `instance_data`.
    pub location: ItemInstanceLocation,
    pub quality: ItemInstanceQuality,
    pub container_item_id: Option<i64>, // Snowflake ID, referances a `Character`, None when item can't have a signature or wasn't crafted by a character
    pub extra_character_id_0: Option<i64>, // Snowflake ID, referances a `Character`, usually used for crafted item signatures
    pub extra_character_id_1: Option<i64>, // Snowflake ID, referances a `Character`, usually used for tracking who is bound to an item
    pub data: Option<Json<ItemInstanceData>>, // None when item can't have or currently does not have data, Some data prevents stacking
}

#[derive(sqlx::FromRow)]
pub struct ItemCollectionEntry {
    pub id: i64,              // Snowflake ID, alias of rowid
    pub character_id: i64,    // Snowflake ID, referances a `Character`
    pub item_content_id: i64, // Snowflake ID, referances an `Item`
    pub location: ItemCollectionEntryLocation,
    pub quality: ItemInstanceQuality,
}

#[derive(sqlx::FromRow)]
pub struct CompanionCollectionEntry {
    pub id: i64,                                          // Snowflake ID, alias of rowid
    pub character_id: i64,                                // Snowflake ID, referances a `Character`
    pub companion_content_id: i64,                        // Snowflake ID, referances a `Companion`
    pub data: Option<Json<CompanionCollectionEntryData>>, // None when item can't have or currently does not have data, Some data prevents stacking
}

#[derive(sqlx::FromRow)]
pub struct CollectionEntry {
    pub id: i64,           // Snowflake ID, alias of rowid
    pub character_id: i64, // Snowflake ID, referances a `Character`
    pub content_id: i64,   // Snowflake ID, referances an `Unlcok`
}
/* End Game Data Service Schema */

/* Game Content Service Schema */
#[derive(sqlx::FromRow)]
pub struct Asset {
    pub id: i64,           // Snowflake ID, alias of rowid
    pub updated_at: i64,   // Unix timestamp with 10 msec precision
    pub path: String,      // Unique no case
    pub file_type: String, // Must be a valid filetype, needed to understand bianry blob
    pub blob: Vec<u8>,     // Binary blob of file saved to database
}

#[derive(sqlx::FromRow)]
pub struct Content {
    pub id: i64,         // Snowflake ID, alias of rowid
    pub updated_at: i64, // Unix timestamp with 10 msec precision
    pub name: String,    // Unique no case
    pub content_type: ContentType,
    pub content_subtype: ContentSubtype,
    pub data: Json<ContentData>, // None when item does not have extra data
    pub asset_id_0: Option<i64>, // Snowflake ID, referances an `Asset`
    pub asset_id_1: Option<i64>, // Snowflake ID, referances an `Asset`
    pub asset_id_2: Option<i64>, // Snowflake ID, referances an `Asset`
    pub asset_id_3: Option<i64>, // Snowflake ID, referances an `Asset`
}
