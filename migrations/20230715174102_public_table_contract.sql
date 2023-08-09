-- Add migration script here
CREATE TABLE public.contract (
    contract_addr bytea,
    transaction_hash bytea,
    _from bytea NOT NULL,
    input bytea NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(contract_addr)
);
