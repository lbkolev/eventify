-- Add migration script here
CREATE TABLE erc721.approve (
    contract_addr bytea NOT NULL,
    transaction_hash bytea,
    transaction_sender bytea NOT NULL,
    _approved bytea NOT NULL,
    _token_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(transaction_hash),
    CONSTRAINT contract_addr_fk
        FOREIGN KEY(contract_addr)
            REFERENCES public.contract(contract_addr)
);
