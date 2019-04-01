-- Your SQL goes here

CREATE EXTENSION Postgis;

CREATE TABLE organizations (
    id BIGSERIAL PRIMARY KEY,
    display_name VARCHAR(128) NOT NULL,
    is_recipient BOOLEAN NOT NULL,
    is_ready BOOLEAN NOT NULL,
    location GEOGRAPHY(POINT, 4326) NOT NULL,
    created_at TIMESTAMP NOT NULL
);
