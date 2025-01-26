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
