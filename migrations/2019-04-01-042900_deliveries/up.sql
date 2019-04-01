-- Your SQL goes here

CREATE TABLE items (
    id BIGSERIAL PRIMARY KEY,
    item VARCHAR(128) NOT NULL,
    quantity VARCHAR(64) NOT NULL,
    org_id
    expires_at TIMESTAMP
);

CREATE TABLE deliveries (
    id BIGSERIAL PRIMARY KEY,
    from_id BIGSERIAL REFERENCES organizations(id),
    to_id BIGSERIAL REFERENCES organizations(id)
);
