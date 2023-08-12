-- Add migration script here
CREATE TABLE public.transfer (
    contract_addr bytea NOT NULL,
    transaction_hash bytea,
    transaction_sender bytea NOT NULL,
    _to bytea NOT NULL,
    _value bytea NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(transaction_hash),
    CONSTRAINT contract_addr_fk
        FOREIGN KEY(contract_addr)
            REFERENCES public.contract(contract_addr)
);

comment on column public.transfer._to is 'The recipient of the ERC20 transfer';
comment on column public.transfer._value is 'The amount of ERC20 tokens transferred';