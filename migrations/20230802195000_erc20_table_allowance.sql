-- Add migration script here
CREATE TABLE erc20.allowance (
    contract_addr bytea NOT NULL,
    transaction_hash bytea PRIMARY KEY,
    transaction_sender bytea NOT NULL,
    _owner bytea NOT NULL,
    _spender NUMERIC,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

comment on column erc20.allowance._owner is 'The address which owns the funds';
