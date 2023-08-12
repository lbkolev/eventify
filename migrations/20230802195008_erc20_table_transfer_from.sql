-- Add migration script here
CREATE TABLE public.transfer_from (
    contract_addr bytea NOT NULL,
    transaction_hash bytea,
    transaction_sender bytea NOT NULL,
    _from bytea NOT NULL,
    _to bytea NOT NULL,
    _value bytea NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(transaction_hash),
    CONSTRAINT contract_addr_fk
        FOREIGN KEY(contract_addr)
            REFERENCES public.contract(contract_addr)
);

comment on COLUMN public.transfer_from._from is 'The address which owns the funds';
comment on COLUMN public.transfer_from._to is 'The address which will receive the funds';
comment on COLUMN public.transfer_from._value is 'Since we are targeting mainly ERC20 & ERC721, this would be either the amount of funds or the amount of the transferred token';