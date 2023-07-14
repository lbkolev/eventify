-- Add migration script here
CREATE TABLE blocks (
    id SERIAL PRIMARY KEY,
    block_id VARCHAR(255) NOT NULL,
    block_number INTEGER NOT NULL,
    block_hash VARCHAR(255) NOT NULL,
    parent_hash VARCHAR(255) NOT NULL,
    extrinsics_hash VARCHAR(255) NOT NULL,
    state_root VARCHAR(255) NOT NULL,
    digest VARCHAR(255) NOT NULL,
    extrinsics_count INTEGER NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)