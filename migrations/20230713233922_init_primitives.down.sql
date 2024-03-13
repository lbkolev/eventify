-- indexes
DROP INDEX IF EXISTS block_hash_index;
DROP INDEX IF EXISTS block_number;

DROP INDEX IF EXISTS log_address_index;
DROP INDEX IF EXISTS log_topic0_index;
DROP INDEX IF EXISTS log_block_number_index;

-- tables
DROP TABLE IF EXISTS block;
DROP TABLE IF EXISTS log;

DROP TYPE IF EXISTS network_type;