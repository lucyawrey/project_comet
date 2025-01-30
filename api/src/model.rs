#![allow(dead_code)]
/* Player Data Structs */
pub struct Player {
    id: i64,          // Snowflake ID, alias of rowid
    updated_at: i64,  // Unix timestamp with 10 msec precision
    username: String, // Unique no case
    role: GameRole,
}

pub struct Credential {
    id: i64,           // Snowflake ID, alias of rowid
    updated_at: i64,   // Unix timestamp with 10 msec precision
    player_id: String, // Snowflake ID, referances a `Player`, unique
    password_hash: String,
}

pub struct Character {
    id: i64,            // Snowflake ID, alias of rowid
    updated_at: i64,    // Unix timestamp with 10 msec precision
    name: String,       // Unique no case with `home_world_id`
    role: GameRole, // Same type as `Player.role`, `Character.role` can be a lower rank than `Player.role` but should never be higher than it.
    home_world_id: i64, // Snowflake ID, referances a `World`
    player_id: i64, // Snowflake ID, referances a `Player`
    ancestry: CharacterAncestry,
    gender: CharacterGender,
    customize_data: CustomizeData,
    gameplay_data: GameplayData,
    quest_data: QuestData,
    roleplaying_data: RoleplayingData,
    npc_relationship_data: NpcRelationshipData,
    gender_data: Option<GenderData>,
}

pub struct CustomizeData {}
pub struct GameplayData {}
pub struct QuestData {}
pub struct RoleplayingData {}
pub struct NpcRelationshipData {}
pub struct GenderData {}

pub struct LogicalServer {
    id: i64,         // String ID, unique primary key
    created_at: i64, // Unix timestamp with 10 msec precision
    updated_at: i64, // Unix timestamp with 10 msec precision
    name: String,
}

pub struct World {
    id: i64,                // Snowflake ID, alias of rowid
    updated_at: i64,        // Unix timestamp with 10 msec precision
    name: String,           // Unique no case
    logical_server: String, // String ID, referances a `LogicalServer`
}

pub struct Guild {
    id: i64,         // Snowflake ID, alias of rowid
    updated_at: i64, // Unix timestamp with 10 msec precision
    name: String,    // Unique no case
}

pub struct GuildMembership {
    id: i64,           // Snowflake ID, alias of rowid
    guild_id: i64,     // Snowflake ID, referances a `Guild`
    character_id: i64, // Snowflake ID, referances a `Character`
    role: GuildRole,
}

pub struct Friendship {
    id: i64,             // Snowflake ID, alias of rowid
    character_1_id: i64, // Snowflake ID, referances a `Character`
    character_2_id: i64, // Snowflake ID, referances a `Character`
}

pub struct ItemInstance {
    id: i64,           // Snowflake ID, alias of rowid
    character_id: i64, // Snowflake ID, referances a `Character`
    item_id: i64,      // Snowflake ID, referances an `Item`
    quantity: i64, // Quantitiy can only be above item's `stack_size` when in a box. `is_unique` items never stack. Items can only stack if they have the same `location`, `quality`, `craft_character_id` and no `instance_data`.
    location: ItemInstanceLocation,
    quality: ItemInstanceQuality,
    craft_character_id: Option<i64>, // Snowflake ID, referances a `Character`, None when item can't have a signature or wasn't crafted by a character
    bound_character_id: Option<i64>, // Snowflake ID, referances a `Character`, None when item can't have a signature or wasn't crafted by a character
    container_item_instance_id: Option<i64>, // Snowflake ID, referances a `Character`, None when item can't have a signature or wasn't crafted by a character
    data: Option<ItemInstanceData>, // None when item can't have or currently does not have data, Some data prevents stacking
}

pub struct ItemInstanceData {}

pub struct ItemCollectionEntry {
    id: i64,           // Snowflake ID, alias of rowid
    character_id: i64, // Snowflake ID, referances a `Character`
    item_id: i64,      // Snowflake ID, referances an `Item`
    location: ItemCollectionEntryLocation,
    quality: ItemInstanceQuality,
}

pub struct CompanionCollectionEntry {
    id: i64,                                    // Snowflake ID, alias of rowid
    character_id: i64,                          // Snowflake ID, referances a `Character`
    companion_id: i64,                          // Snowflake ID, referances a `Companion`
    data: Option<CompanionCollectionEntryData>, // None when item can't have or currently does not have data, Some data prevents stacking
}

pub struct CompanionCollectionEntryData {}

pub struct UnlockCollectionEntry {
    id: i64,           // Snowflake ID, alias of rowid
    character_id: i64, // Snowflake ID, referances a `Character`
    unlock_id: i64,    // Snowflake ID, referances an `Unlcok`
}

/* Game Content Structs */
pub struct Item {
    id: i64,         // Snowflake ID, alias of rowid
    updated_at: i64, // Unix timestamp with 10 msec precision
    name: String,    // Unique no case
    stack_size: i64,
    item_type: ItemType,
    is_unique: bool, // If true instances of this item never stack
    is_soulbound: bool,
    tradability: ItemTradability,
    data: Option<ItemData>,     // None when item does not have extra data
    icon_asset: Option<String>, // Game asset referance, None means use default icon
    drop_model_asset: Option<String>, // Game asset referance, None means use drop model
    actor_asset: Option<String>, // Game asset referance, None means item has no non drop model actor or an actor is not implemented yet
}

pub struct ItemData {}

pub struct Companion {
    id: i64,         // Snowflake ID, alias of rowid
    updated_at: i64, // Unix timestamp with 10 msec precision
    name: String,    // Unique no case
    companion_type: CompanionType,
    data: Option<CompanionData>, // Some or None depends on `companion_type`
    icon_asset: Option<String>,  // Game asset referance, None means use default icon
    actor_asset: Option<String>, // Game asset referance, None means actor is not implemented yet
}

pub struct CompanionData {}

pub struct Unlock {
    id: i64,                    // Snowflake ID, alias of rowid
    updated_at: i64,            // Unix timestamp with 10 msec precision,
    name: String,               // Unique no case
    unlock_type: UnlockType,    // DEFAULT 0 NOT None, -- Enum(Todo=0)
    data: Option<UnlockData>,   // Some or None depends on `unlock_type`
    icon_asset: Option<String>, // Game asset referance, None means use default icon
}

pub struct UnlockData {}

/* Integer Enuns */
pub enum GameRole {
    NewPlayer = 0,
    Player = 1,
    MembershipPlayer = 2,
    GM = 3,
    Admin = 4,
}

pub enum CredentialType {
    Password = 0,
    RecoveryCode = 1,
    AccessToken = 2,
}

pub enum CharacterAncestry {
    Cat = 0,
    Human = 1,
}

pub enum CharacterGender {
    Neutral = 0,   // they/them
    Feminine = 1,  // she/her
    Masculine = 2, // he/him
    None = 3,      // it/it's
    Fluid = 4, // based on current presentation--active glamour and customize_data from either your base character or current class.
    Advanced = 5, // custom pronouns
}

pub enum GuildRole {
    Member = 0,
    Trustee = 1,
}

pub enum ItemInstanceLocation {
    Other = 0,
    Dropped = 1,
    Inventory = 2,
    Equipped = 3,
    InventoryContainer = 4,
    ClassCrystal = 5,
    Box = 6,
}

pub enum ItemInstanceQuality {
    Normal = 0,
    Silver = 1,
    Gold = 2,
}

pub enum ItemCollectionEntryLocation {
    NotTracked = 0,
    Soulbound = 1,
    OnCharacter = 2,
    ClassCrystal = 3,
    Box = 4,
}

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

pub enum ItemTradability {
    Untradeable = 0,
    Droppable = 1,
    NpcTradable = 2,
    PlayerTradable = 3,
    PlayerMarketable = 4,
}

pub enum CompanionType {}

pub enum UnlockType {}
