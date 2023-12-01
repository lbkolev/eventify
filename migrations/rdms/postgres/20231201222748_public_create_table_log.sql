-- Add migration script here
CREATE TABLE public.log (
    id SERIAL,
    address BYTEA NOT NULL,
    topics TEXT[] NOT NULL,
    data BYTEA NOT NULL,
    block_hash BYTEA,
    block_number BIGINT,
    transaction_hash BYTEA,
    transaction_index INTEGER,
    log_index INTEGER,
    transaction_log_index INTEGER,
    log_type TEXT,
    removed BOOLEAN,

    PRIMARY KEY(id)
)