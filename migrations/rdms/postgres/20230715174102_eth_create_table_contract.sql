CREATE TABLE eth.contract (
    transaction_hash bytea,
    "from" bytea NOT NULL,
    input bytea NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

comment on column eth.contract.transaction_hash is 'Hash of the transaction which created the contract';
comment on column eth.contract.from is 'Creator of the contract';
comment on column eth.contract.input is 'Input data sent along with the transaction. Essentially the bytecode of the contract';