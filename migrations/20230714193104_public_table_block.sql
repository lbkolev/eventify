CREATE TABLE public.block (
    hash BYTEA PRIMARY KEY,
    parent_hash BYTEA UNIQUE NOT NULL,
    uncles_hash BYTEA NOT NULL,
    author BYTEA NOT NULL,
    state_root BYTEA NOT NULL,
    transactions_root BYTEA NOT NULL,
    receipts_root BYTEA NOT NULL,
    number BIGSERIAL,
    gas_used BYTEA,
    gas_limit BYTEA,
    base_fee_per_gas BYTEA,
    timestamp BIGSERIAL,
    difficulty BYTEA,
    total_difficulty BYTEA,
    transactions INTEGER,
    size BYTEA,
    nonce BYTEA
);

comment on column public.block.hash is 'Hash of execution block';
comment on column public.block.parent_hash is 'Hash of the parent block';
comment on column public.block.uncles_hash is 'Uncle blocks are created when two or more miners create blocks at nearly the same time. Only one block can be mined and accepted as canonical on the blockchain. The others are uncle blocks, which are not included but still provide a reward to their miners for the work done.';
comment on column public.block.author is 'Address of the miner who created the block';
comment on column public.block.state_root is 'root hash for the global state after applying changes in this block';
comment on column public.block.transactions_root is 'root hash of the transactions in the payload';
comment on column public.block.receipts_root is 'hash of the transaction receipts trie';
comment on column public.block.gas_used is 'Total amount of gas used by all transactions in this block';
comment on column public.block.gas_limit is 'Maximum amount of gas that can be used by all transactions in this block';
comment on column public.block.base_fee_per_gas is 'The base fee value';
comment on column public.block.transactions is 'Number of transactions in this block';