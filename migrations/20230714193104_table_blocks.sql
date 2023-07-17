-- Add migration script here
CREATE TABLE public.blocks (
    id SERIAL PRIMARY KEY,
    block_number INTEGER UNIQUE NOT NULL,
    block_hash VARCHAR(255) UNIQUE NOT NULL,
    parent_hash VARCHAR(255) NOT NULL,
    uncles_hash VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    state_root VARCHAR(255) NOT NULL,
    transactions_root VARCHAR(255) NOT NULL,
    receipts_root VARCHAR(255) NOT NULL,
    gas_used INTEGER NOT NULL,
    gas_limit INTEGER NOT NULL,
    base_fee_per_gas INTEGER NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    difficulty VARCHAR(255) NOT NULL,
    total_difficulty VARCHAR(255) NOT NULL,
    nonce VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

comment on column blocks.timestamp is 'Timestamp of the block';
comment on column blocks.created_at is 'The time of the record insertion';
