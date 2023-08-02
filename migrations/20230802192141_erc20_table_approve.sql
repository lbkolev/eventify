-- Add migration script here
CREATE TABLE erc20.approve (
    contract_addr bytea NOT NULL,
    transaction_hash bytea PRIMARY KEY,
    transaction_sender bytea NOT NULL,
    _spender bytea NOT NULL,
    _value NUMERIC,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

comment on column erc20.approve._spender is 'The address which will spend the funds';
comment on column erc20.approve._value is 'The amount of ERC20 tokens to allow spending';
