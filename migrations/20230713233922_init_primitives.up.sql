CREATE TYPE network_type AS ENUM (
    'ethereum', 'zksync', 'polygon', 'optimism', 'arbitrum', 'linea', 'avalanche', 'bsc', 'base'
);

/*
 * Tables
 */
CREATE TABLE IF NOT EXISTS block (
    -- core
    network network_type,
    number BIGINT,
    hash BYTEA UNIQUE,
    parent_hash BYTEA,
    mix_digest BYTEA,
    uncle_hash BYTEA,
    receipt_hash BYTEA,
    root BYTEA,
    tx_hash BYTEA,
    coinbase BYTEA,
    nonce BYTEA,
    gas_used BYTEA,
    gas_limit BYTEA,
    difficulty BYTEA,
    extra BYTEA,
    bloom BYTEA,
    time BIGINT,
    --
    -- additional
    withdrawals_hash BYTEA,
    total_difficulty BYTEA,
    base_fee BIGINT,
    parent_beacon_root BYTEA,
    blob_gas_used BYTEA,
    excess_blob_gas BYTEA,
    --
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(hash)
);

CREATE TABLE IF NOT EXISTS log (
    -- core
    network network_type,
    id SERIAL,
    address BYTEA NOT NULL,
    block_hash BYTEA,
    block_number BIGINT,
    data BYTEA,
    log_index INTEGER,
    removed BOOLEAN,
    topic0 BYTEA,
    topic1 BYTEA,
    topic2 BYTEA,
    topic3 BYTEA,
    tx_hash BYTEA,
    tx_index INTEGER,
    --
    -- additional
    l1_batch_number BIGINT,
    tx_log_index INTEGER,
    log_type TEXT,
    --
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    UNIQUE (address, block_hash, tx_hash),
    PRIMARY KEY(id)
);
---

/*
 * Indexes
 */
CREATE INDEX IF NOT EXISTS block_hash_index ON block (hash);
CREATE INDEX IF NOT EXISTS block_number ON block (number);

CREATE INDEX IF NOT EXISTS log_address_index ON log (address);
CREATE INDEX IF NOT EXISTS log_topic0_index ON log (topic0);
CREATE INDEX IF NOT EXISTS log_block_number_index ON log (block_number);
---
