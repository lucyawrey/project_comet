-- Initial Migration for Creating Empty Database tables

-- Testing Purposes
CREATE TABLE todos (
    id               TEXT     NOT NULL PRIMARY KEY,
    created_at       INTEGER  DEFAULT (unixepoch ()) NOT NULL,
    updated_at       INTEGER  DEFAULT (unixepoch ()) NOT NULL,
    description      TEXT     NOT NULL,
    done             INTEGER  NOT NULL DEFAULT 0,
) STRICT;

-- Actual Game Data Tables
CREATE TABLE player (
    id               TEXT     NOT NULL PRIMARY KEY,
    createdAt        INTEGER  DEFAULT (unixepoch ()) NOT NULL,
    updatedAt        INTEGER  DEFAULT (unixepoch ()) NOT NULL,
    groups           TEXT     DEFAULT '[user]' NOT NULL,
    username         TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    email            TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    emailIsVerified  INTEGER  DEFAULT 0 NOT NULL,
) STRICT;

CREATE TABLE credential (
  id TEXT NOT NULL PRIMARY KEY,
  userId TEXT NOT NULL UNIQUE REFERENCES "user" ("id"),
  passwordHash TEXT NOT NULL,
) STRICT;
