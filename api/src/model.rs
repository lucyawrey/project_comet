#![allow(dead_code)]

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug)]
pub enum Data<T> {
    Json(String),
    Struct(T),
}

/* User Service Schema */
pub struct User {
    pub id: i64,          // Snowflake ID, alias of rowid
    pub updated_at: i64,  // Unix timestamp with 10 msec precision
    pub username: String, // Unique no case
    pub role: Role,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum Role {
    NewPlayer = 0,
    Player = 1,
    MembershipPlayer = 2,
    GameModerator = 3,
    GameAdministrator = 4,
}

pub struct UserPassword {
    pub id: i64,      // Snowflake ID, alias of rowid
    pub user_id: i64, // Snowflake ID, referances a `User`
    pub password_hash: String,
}

pub struct UserSession {
    pub id: String,      // Hash of the generated user session token
    pub expires_at: i64, // Unix timestamp with 10 msec precision a certain time in the future
    pub user_id: i64,    // Snowflake ID, referances a `User`
}

pub struct UserRecoveryCode {
    pub id: String,   // Hash of the generated user account recovery code
    pub user_id: i64, // Snowflake ID, referances a `User`
    pub temporary: bool,
}
/* End User Service Schema */

/* Administration Service Schema */
pub struct AccessToken {
    id: i64,                   // Snowflake ID, alias of rowid
    access_token_hash: String, // Hash of the generated access token. Token format is: `default|server:gameserverid|admin_IdBase64Representation_secret`
    access_level: AccessLevel,
    game_server_id: String,  // String ID, referances a 'GameServer'
    expires_at: Option<i64>, // Unix timestamp with 10 msec precision a certain time in the future}
}

pub enum AccessLevel {
    Default = 0,
    GameServer = 1,
    Administrator = 2,
}

pub struct GameServer {
    pub id: String, // Case insensitive String ID, should be input in lowercase with no spaces
    pub created_at: i64, // Unix timestamp with 10 msec precision
    pub updated_at: i64, // Unix timestamp with 10 msec precision
    pub region_code: String, // Server location represented by a timezone, using case sensitive tz database identifiers. Ex: 'US/Eastern'
    pub display_name: String, // Server name for end user display
}

pub struct World {
    pub id: String, // Case insensitive String ID, should be input in lowercase with no spaces
    pub created_at: i64, // Unix timestamp with 10 msec precision
    pub updated_at: i64, // Unix timestamp with 10 msec precision
    pub game_server_id: String, // String ID, referances a 'GameServer'
    pub display_name: String, // Server name for end user display
}
/* End Administration Service Schema */

/* Game Data Service Schema */
pub struct Character {
    pub id: i64,               // Snowflake ID, alias of rowid
    pub updated_at: i64,       // Unix timestamp with 10 msec precision
    pub name: String, // Unique no case with `home_world_id`. Character is initially created with a silly random name.
    pub role: Role, // Same type as `User.role`, `Character.role` can be a lower rank than `User.role` but should never be higher than it.
    pub home_world_id: String, // String ID, referances a 'World'
    pub user_id: i64, // Snowflake ID, referances a `User`
    pub ancestry: CharacterAncestry,
    pub gender: CharacterGender,
    pub customize_data: Data<CustomizeData>,
    pub gameplay_data: Data<GameplayData>,
    pub quest_data: Data<QuestData>,
    pub roleplaying_data: Data<RoleplayingData>,
    pub npc_relationship_data: Data<NpcRelationshipData>,
    pub gender_data: Option<Data<GenderData>>,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum CharacterAncestry {
    Cat = 0,
    Human = 1,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum CharacterGender {
    Neutral = 0,   // they/them
    Feminine = 1,  // she/her
    Masculine = 2, // he/him
    None = 3,      // it/it's
    Fluid = 4, // based on current presentation--active glamour and customize_data from either your base character or current class.
    Advanced = 5, // custom pronouns
}

pub struct CustomizeData {}
pub struct GameplayData {}
pub struct QuestData {}
pub struct RoleplayingData {}
pub struct NpcRelationshipData {}
pub struct GenderData {}

pub enum GameOptions {
    User {
        id: i64,         // Snowflake ID, alias of rowid
        updated_at: i64, // Unix timestamp with 10 msec precision
        data: Data<GameOptionsData>,
        user_id: i64, // Snowflake ID, referances a `User`
    },
    Character {
        id: i64,         // Snowflake ID, alias of rowid
        updated_at: i64, // Unix timestamp with 10 msec precision
        data: Data<GameOptionsData>,
        character_id: i64, // Snowflake ID, referances a `Character`
    },
    LocalSystem {
        data: Data<GameOptionsData>,
    },
}

pub struct GameOptionsData {}

pub struct Friendship {
    pub id: i64,             // Snowflake ID, alias of rowid
    pub character_1_id: i64, // Snowflake ID, referances a `Character`
    pub character_2_id: i64, // Snowflake ID, referances a `Character`
}

pub struct Guild {
    pub id: i64,         // Snowflake ID, alias of rowid
    pub updated_at: i64, // Unix timestamp with 10 msec precision
    pub name: String,    // Unique no case
}

pub struct GuildMembership {
    pub id: i64,           // Snowflake ID, alias of rowid
    pub guild_id: i64,     // Snowflake ID, referances a `Guild`
    pub character_id: i64, // Snowflake ID, referances a `Character`
    pub role: GuildRole,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum GuildRole {
    Member = 0,
    Trustee = 1,
}

pub struct ItemInstance {
    pub id: i64,           // Snowflake ID, alias of rowid
    pub character_id: i64, // Snowflake ID, referances a `Character`
    pub item_id: i64,      // Snowflake ID, referances an `Item`
    pub quantity: i64, // Quantitiy can only be above item's `stack_size` when in a box. `is_unique` items never stack. Items can only stack if they have the same `location`, `quality`, `craft_character_id` and no `instance_data`.
    pub location: ItemInstanceLocation,
    pub quality: ItemInstanceQuality,
    pub craft_character_id: Option<i64>, // Snowflake ID, referances a `Character`, None when item can't have a signature or wasn't crafted by a character
    pub bound_character_id: Option<i64>, // Snowflake ID, referances a `Character`, None when item can't have a signature or wasn't crafted by a character
    pub container_item_instance_id: Option<i64>, // Snowflake ID, referances a `Character`, None when item can't have a signature or wasn't crafted by a character
    pub data: Option<Data<ItemInstanceData>>, // None when item can't have or currently does not have data, Some data prevents stacking
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum ItemInstanceLocation {
    Other = 0,
    Dropped = 1,
    NpcMerchant = 2,
    Market = 3,
    Inventory = 4,
    Equipped = 5,
    InventoryContainer = 6,
    ClassCrystal = 7,
    Box = 8,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum ItemInstanceQuality {
    Normal = 0,
    Silver = 1,
    Gold = 2,
}

pub struct ItemInstanceData {}

pub struct ItemCollectionEntry {
    pub id: i64,           // Snowflake ID, alias of rowid
    pub character_id: i64, // Snowflake ID, referances a `Character`
    pub item_id: i64,      // Snowflake ID, referances an `Item`
    pub location: ItemCollectionEntryLocation,
    pub quality: ItemInstanceQuality,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum ItemCollectionEntryLocation {
    NotTracked = 0,
    Soulbound = 1,
    OnCharacter = 2,
    ClassCrystal = 3,
    Box = 4,
}

pub struct CompanionCollectionEntry {
    pub id: i64,                                          // Snowflake ID, alias of rowid
    pub character_id: i64,                                // Snowflake ID, referances a `Character`
    pub companion_id: i64,                                // Snowflake ID, referances a `Companion`
    pub data: Option<Data<CompanionCollectionEntryData>>, // None when item can't have or currently does not have data, Some data prevents stacking
}

pub struct CompanionCollectionEntryData {}

pub struct UnlockCollectionEntry {
    pub id: i64,           // Snowflake ID, alias of rowid
    pub character_id: i64, // Snowflake ID, referances a `Character`
    pub unlock_id: i64,    // Snowflake ID, referances an `Unlcok`
}
/* End Game Data Service Schema */

/* Game Content Service Schema */
pub struct Item {
    pub id: i64,         // Snowflake ID, alias of rowid
    pub updated_at: i64, // Unix timestamp with 10 msec precision
    pub name: String,    // Unique no case
    pub stack_size: i64,
    pub item_type: ItemType,
    pub is_unique: bool, // If true instances of this item never stack
    pub is_soulbound: bool,
    pub tradability: ItemTradability,
    pub data: Option<Data<ItemData>>, // None when item does not have extra data
    pub icon_asset: Option<String>,   // Game asset referance, None means use default icon
    pub drop_model_asset: Option<String>, // Game asset referance, None means use drop model
    pub actor_asset: Option<String>, // Game asset referance, None means item has no non drop model actor or an actor is not implemented yet
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum ItemType {
    Currency = 0,
    Material = 1,
    Consumable = 2,
    QuestItem = 3,
    UnlockItem = 4,
    Equipment = 5,
    InventoryContainer = 6,
    ClassCrystal = 7,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum ItemTradability {
    Untradeable = 0,
    Droppable = 1,
    NpcTradable = 2,
    PlayerTradable = 3,
    PlayerMarketable = 4,
}

pub struct ItemData {}

pub struct Companion {
    pub id: i64,         // Snowflake ID, alias of rowid
    pub updated_at: i64, // Unix timestamp with 10 msec precision
    pub name: String,    // Unique no case
    pub companion_type: CompanionType,
    pub data: Option<Data<CompanionData>>, // Some or None depends on `companion_type`
    pub icon_asset: Option<String>,        // Game asset referance, None means use default icon
    pub actor_asset: Option<String>, // Game asset referance, None means actor is not implemented yet
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum CompanionType {
    Mount = 0,
}

pub struct CompanionData {}

pub struct Unlock {
    pub id: i64,                        // Snowflake ID, alias of rowid
    pub updated_at: i64,                // Unix timestamp with 10 msec precision,
    pub name: String,                   // Unique no case
    pub unlock_type: UnlockType,        // DEFAULT 0 NOT None
    pub data: Option<Data<UnlockData>>, // Some or None depends on `unlock_type`
    pub icon_asset: Option<String>,     // Game asset referance, None means use default icon
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum UnlockType {
    Hairstyle = 0,
}

pub struct UnlockData {}
/* End Game Content Service Schema */
