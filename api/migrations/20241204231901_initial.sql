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
    data               TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    UNIQUE(name, home_world_id)
) STRICT;

CREATE TABLE class (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    character_id       INTEGER  NOT NULL REFERENCES character(id),
    content_class_id   INTEGER  NOT NULL REFERENCES content(id),
    experience         INTEGER  DEFAULT 0 NOT NULL,
    level              INTEGER  DEFAULT 1 NOT NULL,
    max_hp             INTEGER  DEFAULT 10 NOT NULL,
    is_unlocked        INTEGER  DEFAULT TRUE NOT NULL -- Boolean
    customize_data     TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    data               TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    class_container_item_id     INTEGER  REFERENCES item(id), -- NULL if game has no class containers or if this class does not require one
    UNIQUE(character_id, content_class_id),
) STRICT;

CREATE TABLE character_status (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    character_id       INTEGER  NOT NULL UNIQUE REFERENCES character(id),
    active_class_id    INTEGER  NOT NULL REFERENCES content(id),

    data               TEXT     DEFAULT "{}" NOT NULL, -- JSON object
    active_class_container_id INTEGER  REFERENCES content(id), -- NULL if active class is base class or class is being used without a ClassContainer
    active_fashion_container_id INTEGER  REFERENCES content(id), -- NULL if no fashion container is equipped
    UNIQUE(character_id, content_class_id),
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
    name               TEXT     NOT NULL COLLATE NOCASE, -- Case insensitive indexed name, should be between 2 and 30 legal characters with at most 4 spaces
    home_world_id      TEXT     NOT NULL REFERENCES world(id),
    UNIQUE(name, home_world_id)
) STRICT;

CREATE TABLE guild_membership (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    guild_id           INTEGER  NOT NULL REFERENCES guild(id),
    character_id       INTEGER  NOT NULL REFERENCES character(id),
    role               INTEGER  DEFAULT 0 NOT NULL, -- Enum(Member=0, Trustee=1)
    UNIQUE(guild_id, character_id)
) STRICT;

CREATE TABLE item (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    character_id       INTEGER  NOT NULL REFERENCES character(id),
    content_item_id    INTEGER  NOT NULL REFERENCES content(id),
    quantity           INTEGER  DEFAULT 1 NOT NULL, -- Tracks quantity of non-unique, non container item. Quanitity cannot exeed an item's `stack_size` unless it is in the Box. Otherwise, a second instance will need to be made for a new stack. Item instances can be merged if they have the same `location`, `quality`, `craft_character_id`, no `instance_data`, and the new total quantity is legal. Instances of `is_unique` items can never merge or have a quantity other than 1 even in the Box. When two instances are merged the one target location is prioritized first and the one with the older ID is prioritized next. The other instance is deleted. Instances with quanitity 0 are deleted. Tracks number of contained instances for ClassContainer and InventoryContainers (when in Inventory).
    location           INTEGER  DEFAULT 3 NOT NULL, -- Enum(Other=0, Dropped=1, NpcMerchant=2, Market=3, Inventory=4, Equipped=5, InventoryContainer=6, ClassContainer=7, Box=8)
    quality            INTEGER  DEFAULT 0 NOT NULL, -- Enum(Normal=0, Silver=1, Gold=2)
    character_id_2     INTEGER  REFERENCES character(id), -- NULL when item can't have a signature or wasn't crafted by a user
    character_id_3     INTEGER  REFERENCES character(id), -- NULL when item can't be or currently is not Soulbound. Soulbound items will always return to bound character
    container_item_id  INTEGER  REFERENCES item(id), -- NULL when item can't have a signature or wasn't crafted by a user
    data               TEXT -- JSON object, NULL when item can't have or currently does not have data, Non-NULL data prevents stacking
) STRICT;
CREATE INDEX item_character_id_index ON item(character_id);

CREATE TABLE item_collection_entry (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    character_id       INTEGER  NOT NULL REFERENCES character(id),
    content_item_id    INTEGER  NOT NULL REFERENCES content(id),
    location           INTEGER  DEFAULT 0 NOT NULL, -- Enum(NotTracked=0, Soulbound=1, OnCharacter=2, ClassContainer=3, Box=4), a location of NotTracked means this character had the item at some point (because the collection entry exists) but either no longer owns at least 1 instance of it or chose to not add it to their collection. NotTracked entries will have their `quality` set to Normal and will not be updated at all until the item is anually added to the collection. These lost items may be able to be duplicated or reobtained. A location of Soulbound means the item is Soulbound but not currently in the binder's possession.
    quality            INTEGER  DEFAULT 0 NOT NULL, -- Enum(Normal=0, Silver=1, Gold=2), highest quality of item in the highest priority localtion
    UNIQUE(character_id, content_item_id)
) STRICT;
CREATE INDEX item_collection_entry_character_id_index ON item_collection_entry(character_id);

CREATE TABLE companion_collection_entry (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    character_id       INTEGER  NOT NULL REFERENCES character(id),
    content_companion_id        INTEGER  NOT NULL REFERENCES content(id),
    data               TEXT, -- JSON object, NULL when companion does not have special data (e.g. modified color or name)
    UNIQUE(character_id, content_companion_id)
) STRICT;
CREATE INDEX companion_collection_entry_character_id_index ON companion_collection_entry(character_id);

CREATE TABLE collection_entry (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    character_id       INTEGER  NOT NULL REFERENCES character(id),
    content_id         INTEGER  NOT NULL REFERENCES content(id),
    UNIQUE(character_id, content_id)
) STRICT;
CREATE INDEX collection_entry_character_id_index ON collection_entry(character_id);
-- End Game Data Service Schema

-- Game Content Service Schema
CREATE TABLE asset (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    path               TEXT     NOT NULL UNIQUE COLLATE NOCASE, -- Case insensitive indexed name, should be a valid unix filesystem path with no spaces, used in the fake asset filesystem
    file_type          TEXT     NOT NULL, -- Must be a valid filetype, needed to understand bianry blob
    blob               BLOB -- Binary blob of file saved to database
) STRICT;

CREATE TABLE content (
    id                 INTEGER  NOT NULL PRIMARY KEY, -- Snowflake ID, alias of rowid
    updated_at         INTEGER  NOT NULL, -- Unix timestamp with 10 msec precision
    name               TEXT     NOT NULL COLLATE NOCASE, -- Case insensitive indexed name, should be between 2 and 30 legal characters with at most 4 spaces
    content_type       INTEGER  DEFAULT 0 NOT NULL, -- Enum(Item=0, Companion=1, Unlock=2)
    content_subtype    INTEGER  DEFAULT 0 NOT NULL, -- Enum(Variable=X)
    data               TEXT     DEFAULT "{}" NOT NULL,
    asset_id           INTEGER  REFERENCES asset(id),
    asset_id_2         INTEGER  REFERENCES asset(id),
    asset_id_3         INTEGER  REFERENCES asset(id),
    UNIQUE(name, content_type)
) STRICT;
-- End Game Content Service Schema
