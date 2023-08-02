-- Add migration script here
CREATE TABLE erc20.transfer_from (
    contract_addr bytea NOT NULL,
    transaction_hash bytea PRIMARY KEY,
    transaction_sender bytea NOT NULL,
    _from bytea NOT NULL,
    _to bytea NOT NULL,
    _value NUMERIC,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

comment on column erc20.transfer_from._from is 'The address which owns the funds';
