-- Your SQL goes here

CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(100) NOT NULL,
    password VARCHAR(128) NOT NULL,
    created_at TIMESTAMP NOT NULL
);
