-- Add migration script here
CREATE TABLE IF NOT EXISTS users
(
    id       BIGSERIAL PRIMARY KEY,
    username TEXT UNIQUE NOT NULL
);
