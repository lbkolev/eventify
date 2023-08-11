## Chainthru

- [Chainthru](#chainthru)
- [Database schema](#database-schema)
- [Crates](#crates)
- [Documentation](#documentation)
  - [Indexer](#indexer)
  - [Server](#server)


## Database schema
```mermaid
erDiagram 

"public.block" {
    bytea hash "PK"
    bytea parent_hash "Hash of the parent block"
    bytea uncles_hash 
    bytea author "Address of the miner who created the block"
    bytea state_root
    bytea transactions_root
    bytea receipts_root
    BIGSERIAL number
    bytea gas_used
    bytea gas_limit
    bytea base_fee_per_gas
    BIGSERIAL timestamp
    bytea difficulty
    bytea total_difficulty
    INTEGER transactions "Number of transactions in the block"
    bytea size
    bytea nonce
}

"public.contract" {
    bytea contract_addr "PK"
    bytea transaction_hash
    bytea _from
    bytea input
    TIMESTAMP created_at "The TS of the table entry"
}

"erc20.transfer" ||--o{ "public.contract": "depends on"
"erc20.transfer" {
    bytea contract_addr "FK"
    bytea transaction_hash "PK"
    bytea transaction_sender
    bytea _to
    bytea _value
    TIMESTAMP created_at "The TS of the table entry"
}

"erc20.transfer_from" ||--o{ "public.contract": "depends on"
"erc20.transfer_from" {
    bytea contract_addr "FK"
    bytea transaction_hash "PK"
    bytea transaction_sender
    bytea _from
    bytea _to
    bytea _value
    TIMESTAMP created_at "The TS of the table entry"
}

"erc20.approve" ||--o{ "public.contract": "depends on"
"erc20.approve" {
    bytea contract_addr "FK"
    bytea transaction_hash "PK"
    bytea transaction_sender
    bytea _spender
    bytea _value
    TIMESTAMP created_at "The TS of the table entry"
}

"erc721.transfer_from" ||--o{ "public.contract": "depends on"
"erc721.transfer_from" {
    bytea contract_addr "FK"
    bytea transaction_hash "PK"
    bytea transaction_sender
    bytea _from
    bytea _to
    bytea _token_id
    TIMESTAMP created_at "The TS of the table entry"
}

"erc721.safe_transfer_from" ||--o{ "public.contract": "depends on"
"erc721.safe_transfer_from" {
    bytea contract_addr "FK"
    bytea transaction_hash "PK"
    bytea transaction_sender
    bytea _from
    bytea _to
    bytea _token_id
    TIMESTAMP created_at "The TS of the table entry"
}

"erc721.approve" ||--o{ "public.contract": "depends on"
"erc721.approve" {
    bytea contract_addr "FK"
    bytea transaction_hash "PK"
    bytea transaction_sender
    bytea _approved
    bytea _token_id
    TIMESTAMP created_at "The TS of the table entry"
}
```

## Crates

## Documentation
Documentation is segregated in two:
- one part is 

### Indexer
- [Introductory & Architectural](./docs/chainthru-index.md)
- [Crate level/functionality](...)

### Server
- [Introductory & Architectural](./docs/chainthru-api.md)
- [Crate level/functionality](...)

