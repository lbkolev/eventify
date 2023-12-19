CREATE TABLE eth.transaction (
    hash BYTEA,
    nonce BYTEA,
    gas BYTEA,
    gas_price BYTEA,
    max_fee_per_gas BYTEA,
    max_priority_fee_per_gas BYTEA,
    block_hash BYTEA,
    block_number BIGSERIAL,
    transaction_index INTEGER,
    transaction_type INTEGER,
    input BYTEA,
    v INTEGER,
    r BYTEA,
    s BYTEA,
    "from" BYTEA NOT NULL,
    "to" BYTEA,
    value BYTEA,

    PRIMARY KEY(hash)
);

comment on table eth.transaction is 'Contains all the transactions that are not considered special, but we have still got a function signature for';
comment on column eth.transaction.from is 'The address that sent the transaction';
comment on column eth.transaction.to is 'The address that received the transaction';
comment on column eth.transaction.input is 'The input data for the transaction';