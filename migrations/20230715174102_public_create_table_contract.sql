-- Add migration script here
CREATE TABLE public.contract (
    contract_addr bytea,
    transaction_hash bytea,
    _from bytea NOT NULL,
    input bytea NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(contract_addr)
);

comment on column public.contract.contract_addr is 'The address of the contract';
comment on column public.contract.transaction_hash is 'The hash of the transaction which created the contract';
comment on column public.contract._from is 'The address which created the contract';
comment on column public.contract.input is 'The input data sent along with the transaction. Essentially the bytecode of the contract';