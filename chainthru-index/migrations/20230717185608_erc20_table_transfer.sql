-- Add migration script here
CREATE TABLE erc20.transfer (
    id SERIAL PRIMARY KEY,
    transaction_hash bytea UNIQUE NOT NULL,
    contract bytea NOT NULL,
    send_from bytea NOT NULL,
    send_to bytea NOT NULL,
    value NUMERIC,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);