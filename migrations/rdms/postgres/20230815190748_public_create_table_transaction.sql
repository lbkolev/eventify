-- Add migration script here
CREATE TABLE public.transaction (
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
    _from BYTEA NOT NULL,
    _to BYTEA,
    value BYTEA,

    PRIMARY KEY(hash)
);

comment on table public.transaction is 'Contains all the transactions that are not considered special, but we have still got a function signature for';
comment on column public.transaction._from is 'The address that sent the transaction';
comment on column public.transaction._to is 'The address that received the transaction';
comment on column public.transaction.input is 'The input data for the transaction';