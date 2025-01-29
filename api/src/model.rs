struct _Player {
    id: i64,
    updated_at: i64,
    username: String,
    email: String,
    email_is_verified: bool,
    role: _PlayerRole,
}

struct _Credential {
    id: i64,
    updated_at: i64,
    player_id: String,
    password_hash: String,
}

struct _Character {
    id: i64,
    updated_at: i64,
    name: String,
    home_world_id: i64,
    player_id: i64,
    guild_id: i64,
    ancestry: _CharacterAncestry,
    gender: _CharacterGender,
    customize_data: _CustomizeData,
    roleplay_data: _RoleplayData,
    quest_data: _QuestData,
    gameplay_data: _GameplayData,
}

pub struct _CustomizeData {}
pub struct _RoleplayData {}
pub struct _QuestData {}
pub struct _GameplayData {}

pub enum _PlayerRole {
    Guest = 0,
    Player = 1,
    Gm = 2,
    Admin = 3,
}

pub enum _CharacterAncestry {
    Cat = 0,
    Human = 1,
}

pub enum _CharacterGender {
    Other = 0,
    Girl = 1,
    Boy = 2,
}

pub enum _ItemInstanceLocation {
    Equipped = 0,
    Inventory = 1,
    InventoryBag = 2,
    Box = 3,
    Dropped = 4,
    Special = 5,
}

pub enum _ItemInstanceQuality {
    Normal = 0,
    Silver = 1,
    Gold = 2,
}

pub enum _ItemType {
    Currency = 0,
    Material = 1,
    Consumable = 2,
    QuestItem = 3,
    UnlockItem = 4,
    Equipment = 5,
    InventoryBag = 6,
    ClassCrystal = 7,
}

pub enum _ItemTradability {
    Untradeable = 0,
    Droppable = 1,
    Tradable = 2,
    Marketable = 3,
}
