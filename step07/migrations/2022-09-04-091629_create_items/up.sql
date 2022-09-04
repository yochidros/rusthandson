-- Your SQL goes here
CREATE TABLE IF NOT EXISTS items(
  id       INTEGER PRIMARY KEY,
  category TEXT NOT NULL,
  price    INTEGER NOT NULL
);