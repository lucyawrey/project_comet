-- Initial Migration for Creating Database Schema

-- Player Data Tables
CREATE TABLE player (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, Alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix Timestamp
    username           TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    email              TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    email_is_verified  INTEGER  DEFAULT 0 NOT NULL, -- Boolean
    role               INTEGER  DEFAULT 0 NOT NULL -- Enum(Guest=0, Player=1, GM=2, Admin=3)
) STRICT;

CREATE TABLE credential (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, Alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix Timestamp
    player_id          TEXT     NOT NULL UNIQUE REFERENCES user(id),
    password_hash      TEXT     NOT NULL
) STRICT;

CREATE TABLE character (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, Alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix Timestamp
    name               TEXT     NOT NULL COLLATE NOCASE,
    home_world_id      INTEGER  NOT NULL REFERENCES world(id),
    player_id          INTEGER  NOT NULL REFERENCES player(id),
    guild_id           INTEGER  REFERENCES guild(id),
    ancestry           INTEGER  DEFAULT 0 NOT NULL, -- Enum(cat=0, human=1)
    gender             INTEGER  DEFAULT 0 NOT NULL, -- Enum(other=0, girl=1, boy=2)
    customize_data     TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    roleplay_data      TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    quest_data         TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    gameplay_data      TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    UNIQUE(name, home_world_id)
) STRICT;

CREATE TABLE world (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, Alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix Timestamp
    name               TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    logical_server     TEXT     NOT NULL
) STRICT;

CREATE TABLE guild (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, Alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix Timestamp
    name               TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    owner_player_id    INTEGER  NOT NULL REFERENCES player(id)
) STRICT;

CREATE TABLE friendship (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, Alias of rowid
    character_1_id     INTEGER  NOT NULL REFERENCES character(id),
    character_2_id     INTEGER  NOT NULL REFERENCES character(id)
) STRICT;
CREATE UNIQUE INDEX uqique_friendship_index ON friendship(character_1_id, character_2_id);

CREATE TABLE item_instance (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, Alias of rowid
    character_id       INTEGER  NOT NULL REFERENCES character(id),
    item_id            INTEGER  NOT NULL REFERENCES item(id),
    quantity           INTEGER  DEFAULT 1 NOT NULL, -- Quantitiy can only be above item's `stack_size` when in a box. `is_unique` items never stack. Items can only stack if they have the same `location`, `quality`, `craft_character_id` and no `instance_data`.
    location           INTEGER  DEFAULT 0 NOT NULL, -- Enum(Equipped=0, Inventory=1, InventoryBag=2, Box=3, Dropped=4, Special=5)
    quality            INTEGER  DEFAULT 0 NOT NULL, -- Enum(Normal=0, Silver=1, Gold=2)
    part_of_collection INTEGER  DEFAULT 0 NOT NULL, -- Boolean
    has_data           INTEGER  DEFAULT 0 NOT NULL, -- Boolean
    craft_character_id INTEGER  REFERENCES character(id), -- NULL when item can't have a signature or wasn't crafted by a player.
    instance_data      TEXT -- JSON object, TEXT or NULL depends on `has_data`
) STRICT;

-- Game Content Tables
CREATE TABLE item (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, Alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix Timestamp
    name               TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    stack_size         INTEGER  DEFAULT 1 NOT NULL,
    is_unique          INTEGER  DEFAULT 0 NOT NULL, -- Boolean
    type               INTEGER  DEFAULT 0 NOT NULL, -- Enum(Currency=0, Material=1, Consumable=2, QuestItem=3, UnlockItem=4, Equipment=5, InventoryBag = 6, ClassCrystal = 7)
    tradability        INTEGER  DEFAULT 1 NOT NULL, -- Enum(Untradeable=0, Droppable=1, Tradable=2, Marketable=3)
    item_data          TEXT, -- JSON object, TEXT or NULL depends on `type`
    icon_path          TEXT, -- Relative game asset path, NULL means use default icon
    drop_model_path    TEXT -- Relative game asset path, NULL means use drop model
) STRICT;
