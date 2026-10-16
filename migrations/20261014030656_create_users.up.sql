CREATE TABLE users
(
    id            UUID PRIMARY KEY      DEFAULT uuidv7(),
    username      TEXT         NOT NULL UNIQUE,
    email         TEXT         NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at    timestamptz  NOT NULL DEFAULT NOW(),
    updated_at    timestamptz  NOT NULL DEFAULT NOW()
)