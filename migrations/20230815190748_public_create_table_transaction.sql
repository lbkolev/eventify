-- Add migration script here
CREATE TABLE public.transaction (
    hash BYTEA PRIMARY KEY,
    _from bytea NOT NULL,
    _to bytea NOT NULL,
    input bytea,

    PRIMARY KEY(hash)
);

comment on table public.transaction is "Contains all the transactions that are not considered special, but we've still got a function signature for."
comment on column public.transaction._from is "The address that sent the transaction."
comment on column public.transaction._to is "The address that received the transaction."
comment on column public.transaction.input is "The input data for the transaction."