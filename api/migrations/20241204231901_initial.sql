-- Initial Migration for Creating Database Schema

-- User Service Schema
CREATE TABLE user (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision, updates when a login method is updated
    username           TEXT     NOT NULL UNIQUE COLLATE NOCASE, -- Case insensitive indexed name, should be between 2 and 20 legal characters with no whitespace
    role               INTEGER  DEFAULT 0 NOT NULL -- Enum(NewPlayer=0, Player=1, MembershipPlayer=2, GameModerator=3, GameAdministrator=4)
) STRICT;

CREATE TABLE user_password (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    user_id            INTEGER  NOT NULL UNIQUE REFERENCES user(id),
    password_hash      TEXT     NOT NULL
) STRICT;

CREATE TABLE user_session (
    id                 TEXT     NOT NULL PRIMARY KEY, -- Hash of the generated user session token
    expires_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision a certain time in the future
    user_id            INTEGER  NOT NULL REFERENCES user(id)
) STRICT;

CREATE TABLE user_recovery_code (
    id                 TEXT     NOT NULL PRIMARY KEY, -- Hash of the generated user account recovery code
    user_id            INTEGER  NOT NULL UNIQUE REFERENCES user(id),
    temporary          INTEGER  DEFAULT FALSE NOT NULL -- Boolean
) STRICT;
-- End User Service Schema

-- Administration Service Schema
CREATE TABLE access_token (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    access_token_hash  TEXT     NOT NULL, -- Hash of the generated access token. Token format is: `default|server:gameserverid|admin_IdBase64Representation_secret`
    access_level       INTEGER  NOT NULL, -- Enum(Default=0, GameServer=1, Administrator=2)
    game_server_id     TEXT     REFERENCES game_server(id), -- NULL when access_level is not `GameServer`
    expires_at         INTEGER -- Unix timestamp with 10 msec precision a certain time in the future. If NULL, token does not expire
) STRICT;

CREATE TABLE game_server (
    id                 TEXT     NOT NULL PRIMARY KEY COLLATE NOCASE, -- Case insensitive String ID, should be lowercase, short, and have no whitespace or special characters
    created_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    region_code        TEXT     NOT NULL, -- Server location represented by a timezone, using case sensitive tz database identifiers. Ex: 'US/Eastern'
    display_name       TEXT     NOT NULL -- Server name for end user display
) STRICT;

CREATE TABLE world (
    id                 TEXT     NOT NULL PRIMARY KEY COLLATE NOCASE, -- Case insensitive String ID, should be lowercase, short, and have no whitespace or special characters
    created_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    game_server_id     TEXT     NOT NULL REFERENCES game_server(id),
    display_name       TEXT     NOT NULL -- World name for end user display
) STRICT;
CREATE INDEX world_game_server_id_index ON game_server(id);
-- End Administration Service Schema

-- Game Data Service Schema
CREATE TABLE character (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    name               TEXT     NOT NULL COLLATE NOCASE, -- Case insensitive indexed name, should be between 2 and 30 legal characters with at most 4 spaces
    role               INTEGER  DEFAULT 0 NOT NULL, -- Enum(NewPlayer=0, Player=1, MembershipPlayer=2, GameModerator=3, GameAdministrator=4)
    home_world_id      TEXT     NOT NULL REFERENCES world(id),
    user_id            INTEGER  NOT NULL REFERENCES user(id),
    ancestry           INTEGER  DEFAULT 0 NOT NULL, -- Enum(Cat=0, Human=1)
    gender             INTEGER  DEFAULT 0 NOT NULL, -- Enum(Neutral=0, Feminine=1, Masculine=2, None=3, Fluid=4, Advanced=5)
    customize_data     TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    gameplay_data      TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    quest_data         TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    roleplaying_data   TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    npc_relationship_data       TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    gender_data        TEXT, -- JSON object, NULL when gender is not Fluid or Advanced
    UNIQUE(name, home_world_id)
) STRICT;

CREATE TABLE game_options (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    data               TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    user_id            INTEGER  UNIQUE REFERENCES user(id),
    character_id       INTEGER  UNIQUE REFERENCES character(id),
    CHECK((user_id IS NOT NULL AND character_id IS NULL) OR (user_id IS NULL AND character_id IS NOT NULL))
) STRICT;

CREATE TABLE friendship (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    character_1_id     INTEGER  NOT NULL REFERENCES character(id),
    character_2_id     INTEGER  NOT NULL REFERENCES character(id),
    UNIQUE(character_1_id, character_2_id)
) STRICT;

CREATE TABLE guild (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    name               TEXT     NOT NULL UNIQUE COLLATE NOCASE -- Case insensitive indexed name, should be between 2 and 30 legal characters with at most 4 spaces
) STRICT;

CREATE TABLE guild_membership (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    guild_id           INTEGER  NOT NULL REFERENCES guild(id),
    character_id       INTEGER  NOT NULL REFERENCES character(id),
    role               INTEGER  DEFAULT 0 NOT NULL, -- Enum(Member=0, Trustee=1)
    UNIQUE(guild_id, character_id)
) STRICT;

CREATE TABLE item_instance (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    character_id       INTEGER  NOT NULL REFERENCES character(id),
    item_id            INTEGER  NOT NULL REFERENCES item(id),
    quantity           INTEGER  DEFAULT 1 NOT NULL, -- Tracks quantity of non-unique, non container item. Quanitity cannot exeed an item's `stack_size` unless it is in the Box. Otherwise, a second instance will need to be made for a new stack. Item instances can be merged if they have the same `location`, `quality`, `craft_character_id`, no `instance_data`, and the new total quantity is legal. Instances of `is_unique` items can never merge or have a quantity other than 1 even in the Box. When two instances are merged the one target location is prioritized first and the one with the older ID is prioritized next. The other instance is deleted. Instances with quanitity 0 are deleted. Tracks number of contained instances for ClassCrystals and InventoryContainers (when in Inventory).
    location           INTEGER  DEFAULT 3 NOT NULL, -- Enum(Other=0, Dropped=1, NpcMerchant=2, Market=3, Inventory=4, Equipped=5, InventoryContainer=6, ClassCrystal=7, Box=8)
    quality            INTEGER  DEFAULT 0 NOT NULL, -- Enum(Normal=0, Silver=1, Gold=2)
    craft_character_id INTEGER  REFERENCES character(id), -- NULL when item can't have a signature or wasn't crafted by a user
    bound_character_id INTEGER  REFERENCES character(id), -- NULL when item can't be or currently is not Soulbound. Soulbound items will always return to bound character
    container_item_instance_id  INTEGER  REFERENCES item_instance(id), -- NULL when item can't have a signature or wasn't crafted by a user
    data               TEXT -- JSON object, NULL when item can't have or currently does not have data, Non-NULL data prevents stacking
) STRICT;
CREATE INDEX item_instance_character_id_index ON item_instance(character_id);

CREATE TABLE item_collection_entry (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    character_id       INTEGER  NOT NULL REFERENCES character(id),
    item_id            INTEGER  NOT NULL REFERENCES item(id),
    location           INTEGER  DEFAULT 0 NOT NULL, -- Enum(NotTracked=0, Soulbound=1, OnCharacter=2, ClassCrystal=3, Box=4), a location of NotTracked means this character had the item at some point (because the collection entry exists) but either no longer owns at least 1 instance of it or chose to not add it to their collection. NotTracked entries will have their `quality` set to Normal and will not be updated at all until the item is anually added to the collection. These lost items may be able to be duplicated or reobtained. A location of Soulbound means the item is Soulbound but not currently in the binder's possession.
    quality            INTEGER  DEFAULT 0 NOT NULL, -- Enum(Normal=0, Silver=1, Gold=2), highest quality of item in the highest priority localtion
    UNIQUE(character_id, item_id)
) STRICT;
CREATE INDEX item_collection_entry_character_id_index ON item_collection_entry(character_id);

CREATE TABLE companion_collection_entry (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    character_id       INTEGER  NOT NULL REFERENCES character(id),
    companion_id       INTEGER  NOT NULL REFERENCES companion(id),
    data               TEXT, -- JSON object, NULL when companion does not have special data (e.g. modified color or name)
    UNIQUE(character_id, companion_id)
) STRICT;
CREATE INDEX companion_collection_entry_character_id_index ON companion_collection_entry(character_id);

CREATE TABLE unlock_collection_entry (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    character_id       INTEGER  NOT NULL REFERENCES character(id),
    unlock_id          INTEGER  NOT NULL REFERENCES unlock(id),
    UNIQUE(character_id, unlock_id)
) STRICT;
CREATE INDEX unlock_collection_entry_character_id_index ON unlock_collection_entry(character_id);
-- End Game Data Service Schema

-- Game Content Service Schema
CREATE TABLE item (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    name               TEXT     NOT NULL UNIQUE COLLATE NOCASE, -- Case insensitive indexed name, should be between 2 and 30 legal characters with at most 4 spaces
    stack_size         INTEGER  DEFAULT 1 NOT NULL,
    item_type          INTEGER  DEFAULT 0 NOT NULL, -- Enum(Currency=0, Material=1, Consumable=2, QuestItem=3, UnlockItem=4, Equipment=5, InventoryContainer = 6, ClassCrystal = 7)
    is_unique          INTEGER  DEFAULT FALSE NOT NULL, -- Boolean
    is_soulbound       INTEGER  DEFAULT FALSE NOT NULL, -- Boolean
    tradability        INTEGER  DEFAULT 1 NOT NULL, -- Enum(Untradeable=0, Droppable=1, NpcTradable=2, PlayerTradable=3, PlayerMarketable=4)
    sell_value         INTEGER  DEFAULT 1 NOT NULL, -- Value in Gold, ignoring game price change mechanics. Item may not actually be exchangeable for Gold
    data               TEXT, -- JSON object, TEXT or NULL depends on `item_type`
    icon_asset         TEXT, -- Game asset referance, NULL means use default icon
    drop_model_asset   TEXT, -- Game asset referance, NULL means use default drop model
    actor_asset        TEXT -- Game asset referance, NULL means item has no non drop model actor or an actor is not implemented yet
) STRICT;

CREATE TABLE companion (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    name               TEXT     NOT NULL UNIQUE COLLATE NOCASE, -- Case insensitive indexed name, should be between 2 and 30 legal characters with at most 4 spaces
    companion_type     INTEGER  DEFAULT 0 NOT NULL, -- Enum(Todo=0)
    data               TEXT, -- JSON object, TEXT or NULL depends on `companion_type`
    icon_asset         TEXT, -- Game asset referance, NULL means use default icon
    actor_asset        TEXT  -- Game asset referance, NULL means actor is not implemented yet
) STRICT;

CREATE TABLE unlock (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    name               TEXT     NOT NULL UNIQUE COLLATE NOCASE, -- Case insensitive indexed name, should be between 2 and 30 legal characters with at most 4 spaces
    unlock_type        INTEGER  DEFAULT 0 NOT NULL, -- Enum(Todo=0)
    data               TEXT, -- JSON object, TEXT or NULL depends on `unlock_type`
    icon_asset         TEXT -- Game asset referance, NULL means use default icon
) STRICT;
-- End Game Content Service Schema
