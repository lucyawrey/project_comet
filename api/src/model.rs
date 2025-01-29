#![allow(dead_code)]
/* Player Data Structs */
struct Player {
    id: i64,          // Snowflake ID, alias of rowid
    updated_at: i64,  // Unix timestamp with 10 msec precision
    username: String, // Unique no case
    email: String,    // Unique no case
    email_is_verified: bool,
    role: PlayerRole,
}

struct Credential {
    id: i64,           // Snowflake ID, alias of rowid
    updated_at: i64,   // Unix timestamp with 10 msec precision
    player_id: String, // Snowflake ID, referances a `Player`, unique
    password_hash: String,
}

struct Character {
    id: i64,            // Snowflake ID, alias of rowid
    updated_at: i64,    // Unix timestamp with 10 msec precision
    name: String,       // Unique no case with `home_world_id`
    home_world_id: i64, // Snowflake ID, referances a `World`
    player_id: i64,     // Snowflake ID, referances a `Player`
    ancestry: CharacterAncestry,
    gender: CharacterGender,
    customize_data: CustomizeData,
    roleplay_data: RoleplayData,
    quest_data: QuestData,
    gameplay_data: GameplayData,
}

pub struct CustomizeData {}
pub struct RoleplayData {}
pub struct QuestData {}
pub struct GameplayData {}

struct LogicalServer {
    id: i64,         // String ID, unique primary key
    created_at: i64, // Unix timestamp with 10 msec precision
    updated_at: i64, // Unix timestamp with 10 msec precision
    name: String,
}

struct World {
    id: i64,                // Snowflake ID, alias of rowid
    updated_at: i64,        // Unix timestamp with 10 msec precision
    name: String,           // Unique no case
    logical_server: String, // String ID, referances a `LogicalServer`
}

struct Guild {
    id: i64,         // Snowflake ID, alias of rowid
    updated_at: i64, // Unix timestamp with 10 msec precision
    name: String,    // Unique no case
}

struct GuildMembership {
    id: i64,           // Snowflake ID, alias of rowid
    guild_id: i64,     // Snowflake ID, referances a `Guild`
    character_id: i64, // Snowflake ID, referances a `Character`
    role: GuildRole,
}

struct Friendship {
    id: i64,             // Snowflake ID, alias of rowid
    character_1_id: i64, // Snowflake ID, referances a `Character`
    character_2_id: i64, // Snowflake ID, referances a `Character`
}

struct ItemInstance {
    id: i64,           // Snowflake ID, alias of rowid
    character_id: i64, // Snowflake ID, referances a `Character`
    item_id: i64,      // Snowflake ID, referances an `Item`
    quantity: i64, // Quantitiy can only be above item's `stack_size` when in a box. `is_unique` items never stack. Items can only stack if they have the same `location`, `quality`, `craft_character_id` and no `instance_data`.
    location: ItemInstanceLocation,
    quality: ItemInstanceQuality,
    part_of_collection: bool,
    craft_character_id: Option<i64>, // Snowflake ID, referances a `Character`, None when item can't have a signature or wasn't crafted by a character
    data: Option<ItemInstanceData>, // None when item can't have or currently does not have data, Some data prevents stacking
}

pub struct ItemInstanceData {}

/* Game Content Structs */
struct Item {
    id: i64,         // Snowflake ID, alias of rowid
    updated_at: i64, // Unix timestamp with 10 msec precision
    name: String,    // Unique no case
    stack_size: i64,
    is_unique: bool, // If true instances of this item never stack
    typee: ItemType,
    tradability: ItemTradability,
    data: Option<ItemData>,          // None when item does not have extra data
    icon_path: Option<String>,       // Relative game asset path, NULL means use default icon
    drop_model_path: Option<String>, // Relative game asset path, NULL means use drop model
}

struct ItemData {}

/* Integer Enuns */
pub enum PlayerRole {
    Guest = 0,
    Player = 1,
    Gm = 2,
    Admin = 3,
}

pub enum CharacterAncestry {
    Cat = 0,
    Human = 1,
}

pub enum CharacterGender {
    Other = 0,
    Girl = 1,
    Boy = 2,
}

pub enum GuildRole {
    Member = 0,
    Trustee = 1,
}

pub enum ItemInstanceLocation {
    Equipped = 0,
    Inventory = 1,
    InventoryBag = 2,
    Box = 3,
    Dropped = 4,
    Special = 5,
}

pub enum ItemInstanceQuality {
    Normal = 0,
    Silver = 1,
    Gold = 2,
}

pub enum ItemType {
    Currency = 0,
    Material = 1,
    Consumable = 2,
    QuestItem = 3,
    UnlockItem = 4,
    Equipment = 5,
    InventoryBag = 6,
    ClassCrystal = 7,
}

pub enum ItemTradability {
    Untradeable = 0,
    Droppable = 1,
    Tradable = 2,
    Marketable = 3,
}
