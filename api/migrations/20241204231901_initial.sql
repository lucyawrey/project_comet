-- Initial Migration for Creating Empty Database tables
CREATE TABLE player (
    id                 TEXT     NOT NULL PRIMARY KEY, -- UUID
    created_at         INTEGER  DEFAULT (unixepoch()) NOT NULL, -- Unix Timestamp
    updated_at         INTEGER  DEFAULT (unixepoch()) NOT NULL, -- Unix Timestamp
    username           TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    email              TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    email_is_verified  INTEGER  DEFAULT 0 NOT NULL,
    role               INTEGER  DEFAULT 0 NOT NULL -- Enum(Guest=0, Player=1, GM=2, Admin=3)
) STRICT;

CREATE TABLE credential (
    id                   TEXT     NOT NULL PRIMARY KEY, -- UUID
    userId               TEXT     NOT NULL UNIQUE REFERENCES user(id),
    passwordHash         TEXT     NOT NULL
) STRICT;

CREATE TABLE character (
    id                 TEXT     NOT NULL PRIMARY KEY, -- UUID
    createdAt          INTEGER  DEFAULT (unixepoch()) NOT NULL, -- Unix Timestamp
    updatedAt          INTEGER  DEFAULT (unixepoch()) NOT NULL, -- Unix Timestamp
    name               TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    player_id          TEXT     NOT NULL REFERENCES player(id),
    guild_id           TEXT     -- Add guild table
) STRICT;

CREATE TABLE friendship (
    id                 TEXT     NOT NULL PRIMARY KEY, -- UUID
    character_1_id     TEXT     NOT NULL REFERENCES character(id),
    character_2_id     TEXT     NOT NULL REFERENCES character(id)
) STRICT;
CREATE UNIQUE INDEX uqique_friendship_index ON friendship(character_1_id, character_2_id);

CREATE TABLE item_stack (
    id                 TEXT     NOT NULL PRIMARY KEY, -- UUID
    item_id            INTEGER  NOT NULL,
    quantity           INTEGER  NOT NULL,
    location           INTEGER  DEFAULT 0 NOT NULL -- Enum(Inventory=0, Box=1)
) STRICT;
