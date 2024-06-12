-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS todos (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        checked INTEGER NOT NULL DEFAULT 0
    );