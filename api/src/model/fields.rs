use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum Role {
    NewPlayer = 0,
    Player = 1,
    MembershipPlayer = 2,
    GameModerator = 3,
    GameAdministrator = 4,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum AccessLevel {
    Default = 0,
    GameServer = 1,
    Administrator = 2,
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
    Fluid = 4, // based on current presentation--active glamour and customization from either your base character or current class.
    Advanced = 5, // custom pronouns
}

pub struct Customization {
    pub gender_details: GenderDetails,
}
pub struct GenderDetails {}
pub struct CharacterData {
    pub character_history: CharacterHistory,
    pub npc_relationships: NpcRelationships,
}
pub struct CharacterHistory {}
pub struct NpcRelationships {}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum GameOptionsType {
    User = 0,
    Character = 1,
    System = 2,
}

pub struct GameOptionsData {}

pub struct CharacterStatusData {}

pub struct Statistics {}

pub struct ClassData {}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum GuildRole {
    Member = 0,
    Trustee = 1,
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
    ClassItem = 7,
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

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum ItemCollectionEntryLocation {
    NotTracked = 0,
    Soulbound = 1,
    OnCharacter = 2,
    ClassItem = 3,
    Box = 4,
}

pub struct CompanionCollectionEntryData {}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum ContentType {
    Class = 1,

    Item = 0,
    Companion = 100,
    Unlock = 200,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum ContentSubtype {
    Currency = 0,
    Material = 1,
    Consumable = 2,
    QuestItem = 3,
    UnlockItem = 4,
    Equipment = 5,
    InventoryContainer = 6,
    ClassItem = 7,

    Mount = 100,
    Pet = 101,

    Hairstyle = 200,
}

pub enum ContentData {
    Class {},
    Item {
        stack_size: i64,
        is_unique: bool,
        is_soulbound: bool,
        tradability: ItemTradability,
    },
    Companion {},
    Unlock {},
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
