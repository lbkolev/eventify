-- Add migration script here
CREATE TABLE public.log (
    id SERIAL,
    address BYTEA NOT NULL,
    topic0 BYTEA,
    topic1 BYTEA,
    topic2 BYTEA,
    topic3 BYTEA,
    data BYTEA NOT NULL,
    block_hash BYTEA,
    block_number BIGSERIAL,
    transaction_hash BYTEA,
    transaction_index INTEGER,
    transaction_log_index BYTEA,
    log_index BYTEA,
    log_type TEXT,
    removed BOOLEAN,

    PRIMARY KEY(id)
)