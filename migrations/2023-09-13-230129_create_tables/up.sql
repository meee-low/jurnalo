-- Your SQL goes here

CREATE TABLE entries (
    id INTEGER PRIMARY KEY NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    entry_number INTEGER NOT NULL,
    category INTEGER,
    value INTEGER,
    details TEXT
);

CREATE TABLE categories (
    id INTEGER PRIMARY KEY NOT NULL,
    label TEXT NOT NULL UNIQUE,
    prompt TEXT NOT NULL UNIQUE,
    category_type INTEGER NOT NULL
);

CREATE TABLE options (
    id INTEGER PRIMARY KEY NOT NULL,
    label TEXT NOT NULL UNIQUE,
    shortcut TEXT NOT NULL UNIQUE
);

CREATE TABLE category_option (
    id INTEGER PRIMARY KEY NOT NULL,
    category_id INTEGER NOT NULL,
    option_id INTEGER NOT NULL
);
