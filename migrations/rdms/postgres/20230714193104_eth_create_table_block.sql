CREATE TABLE eth.block (
    hash BYTEA,
    parent_hash BYTEA,
    uncles_hash BYTEA,
    author BYTEA,
    state_root BYTEA,
    transactions_root BYTEA,
    receipts_root BYTEA,
    number BIGSERIAL,
    gas_used BYTEA,
    gas_limit BYTEA,
    base_fee_per_gas BYTEA,
    timestamp BIGSERIAL,
    difficulty BYTEA,
    total_difficulty BYTEA,
    size BYTEA,
    nonce BYTEA,

    PRIMARY KEY(hash)
);

comment on column eth.block.hash is 'Hash of execution block';
comment on column eth.block.parent_hash is 'Hash of the parent block';
comment on column eth.block.uncles_hash is 'Uncle blocks are created when two or more miners create blocks at nearly the same time. Only one block can be mined and accepted as canonical on the blockchain. The others are uncle blocks, which are not included but still provide a reward to their miners for the work done.';
comment on column eth.block.author is 'Address of the miner who created the block';
comment on column eth.block.state_root is 'root hash for the global state after applying changes in this block';
comment on column eth.block.transactions_root is 'root hash of the transactions in the payload';
comment on column eth.block.receipts_root is 'hash of the transaction receipts trie';
comment on column eth.block.gas_used is 'Total amount of gas used by all transactions in this block';
comment on column eth.block.gas_limit is 'Maximum amount of gas that can be used by all transactions in this block';
comment on column eth.block.base_fee_per_gas is 'The base fee value';
