
CREATE TABLE public.contract (
    transaction_hash bytea,
    _from bytea NOT NULL,
    input bytea NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

comment on column public.contract.transaction_hash is 'Hash of the transaction which created the contract';
comment on column public.contract._from is 'Creator of the contract';
comment on column public.contract.input is 'Input data sent along with the transaction. Essentially the bytecode of the contract';