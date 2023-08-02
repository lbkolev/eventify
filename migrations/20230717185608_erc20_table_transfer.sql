-- Add migration script here
CREATE TABLE erc20.transfer (
    contract_addr bytea NOT NULL,
    transaction_hash bytea PRIMARY KEY,
    transaction_sender bytea NOT NULL,
    _to bytea NOT NULL,
    _value NUMERIC,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

comment on column erc20.transfer._to is 'The recipient of the ERC20 transfer';
comment on column erc20.transfer._value is 'The amount of ERC20 tokens transferred';