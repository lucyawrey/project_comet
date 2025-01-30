-- Initial Migration for Creating Database Schema

-- Player Data Tables
CREATE TABLE player (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    username           TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    role               INTEGER  DEFAULT 0 NOT NULL -- Enum(NewPlayer=0, Player=1, MembershipPlayer=2, GM=3, Admin=4)
) STRICT;

CREATE TABLE credential (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    player_id          TEXT     NOT NULL UNIQUE REFERENCES user(id),
    credential_type    INTEGER  DEFAULT 0 NOT NULL, -- Enum(Password=0, RecoveryCode=1, OAuth)
    secret_hash        TEXT     NOT NULL,
    UNIQUE(player_id, credential_type)
) STRICT;

CREATE TABLE character (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    name               TEXT     NOT NULL COLLATE NOCASE,
    role               INTEGER  DEFAULT 0 NOT NULL, -- Enum(NewPlayer=0, Player=1, MembershipPlayer=2, GM=3, Admin=4)
    home_world_id      INTEGER  NOT NULL REFERENCES world(id),
    player_id          INTEGER  NOT NULL REFERENCES player(id),
    ancestry           INTEGER  DEFAULT 0 NOT NULL, -- Enum(Cat=0, Human=1)
    gender             INTEGER  DEFAULT 0 NOT NULL, -- Enum(Other=0, Girl=1, Boy=2)
    customize_data     TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    roleplay_data      TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    quest_data         TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    gameplay_data      TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    UNIQUE(name, home_world_id)
) STRICT;

CREATE TABLE logical_server (
    id                 TEXT     NOT NULL PRIMARY KEY COLLATE NOCASE, -- String ID, lowercase with no spaces
    created_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    name               TEXT     NOT NULL -- String name for end user display
) STRICT;

CREATE TABLE world (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    name               TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    logical_server_id  TEXT     NOT NULL REFERENCES logical_server(id)
) STRICT;

CREATE TABLE guild (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    name               TEXT     NOT NULL UNIQUE COLLATE NOCASE
) STRICT;

CREATE TABLE guild_membership (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    guild_id           INTEGER  NOT NULL REFERENCES guild(id),
    character_id       INTEGER  NOT NULL REFERENCES character(id),
    role               INTEGER  DEFAULT 0 NOT NULL -- Enum(Member=0, Trustee=1)
) STRICT;

CREATE TABLE friendship (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    character_1_id     INTEGER  NOT NULL REFERENCES character(id),
    character_2_id     INTEGER  NOT NULL REFERENCES character(id)
) STRICT;
CREATE UNIQUE INDEX uqique_friendship_index ON friendship(character_1_id, character_2_id);

CREATE TABLE item_instance (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    character_id       INTEGER  NOT NULL REFERENCES character(id),
    item_id            INTEGER  NOT NULL REFERENCES item(id),
    quantity           INTEGER  DEFAULT 1 NOT NULL, -- Quantitiy can only be above item's `stack_size` when in a box. `is_unique` items never stack. Items can only stack if they have the same `location`, `quality`, `craft_character_id` and no `instance_data`.
    location           INTEGER  DEFAULT 0 NOT NULL, -- Enum(Equipped=0, Inventory=1, InventoryBag=2, Box=3, Dropped=4, Special=5)
    quality            INTEGER  DEFAULT 0 NOT NULL, -- Enum(Normal=0, Silver=1, Gold=2)
    part_of_collection INTEGER  DEFAULT 0 NOT NULL, -- Boolean
    craft_character_id INTEGER  REFERENCES character(id), -- NULL when item can't have a signature or wasn't crafted by a player.
    data               TEXT -- JSON object, NULL when item can't have or currently does not have data, Non-NULL data prevents stacking
) STRICT;

-- Game Content Tables
CREATE TABLE item (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    name               TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    stack_size         INTEGER  DEFAULT 1 NOT NULL,
    is_unique          INTEGER  DEFAULT 0 NOT NULL, -- Boolean
    item_type          INTEGER  DEFAULT 0 NOT NULL, -- Enum(Currency=0, Material=1, Consumable=2, QuestItem=3, UnlockItem=4, Equipment=5, InventoryContainer = 6, ClassCrystal = 7)
    tradability        INTEGER  DEFAULT 1 NOT NULL, -- Enum(Untradeable=0, Droppable=1, Tradable=2, Marketable=3)
    data               TEXT, -- JSON object, TEXT or NULL depends on `item_type`
    icon_path          TEXT, -- Relative game asset path, NULL means use default icon
    drop_model_path    TEXT -- Relative game asset path, NULL means use drop model
) STRICT;
