# <p align="center">chainthru</p>
<p align="center"> Index Ethereum into a storage backend</p>

<p align="center">
    <a href="https://github.com/lbkolev/chainthru/blob/master/LICENSE">
        <img src="https://img.shields.io/badge/license-MIT-blue.svg">
    </a>
    <a href="https://crates.io/crates/chainthru">
        <img src="https://img.shields.io/crates/v/chainthru.svg">
    </a>
    <a href="https://github.com/lbkolev/chainthru/actions?query=workflow%3ACI+branch%3Amaster">
        <img src="https://github.com/lbkolev/chainthru/actions/workflows/ci.yml/badge.svg">
    </a>
    <a href="https://docs.rs/chainthru">
        <img src="https://img.shields.io/docsrs/chainthru/latest">
    </a>
</p>

> <p align="center"> 🚧 work in progress 🚧<p>

- [chainthru](#chainthru)
  - [Database schema](#database-schema)
  - [Crates](#crates)
  - [Documentation](#documentation)


## Database schema
```mermaid
erDiagram

"public.block" {
    BYTEA hash "PK"
    BYTEA parent_hash "Hash of the parent block"
    BYTEA uncles_hash
    BYTEA author "Address of the miner who created the block"
    BYTEA state_root
    BYTEA transactions_root
    BYTEA receipts_root
    NUMBER number
    BYTEA gas_used
    BYTEA gas_limit
    BYTEA base_fee_per_gas
    BIGSERIAL timestamp
    BYTEA difficulty
    BYTEA total_difficulty
    BYTEA size
    BYTEA nonce
}

"public.contract" {
    BYTEA contract_addr "PK"
    BYTEA transaction_hash
    BYTEA _from
    BYTEA input
    TIMESTAMP created_at "The TS of the table entry"
}

"public.transaction" {
    BYTEA hash "PK"
    BYTEA nonce
    BYTEA block_hash
    NUMBER block_number
    NUMBER transaction_index
    BYTEA _from
    BYTEA _to
    BYTEA gas_price
    BYTEA gas
    BYTEA input
    BYTEA v
    BYTEA r
    BYTEA s
    NUMBER transaction_type
    BYTEA max_fee_per_gas
    BYTEA max_priority_fee_per_gas
}

"public.function_signature" {
    uuid id "PK"
    BYTEA hex_sig
    BYTEA text_sig
}
```

## Crates
Include the following
- [chainthru (binary)](./chainthru/)
- [chainthru-index](./chainthru-index)
- [chainthru-server](./chainthru-server)
- [chainthru-primitives](./chainthru-primitives)

## Documentation

- ### Indexer
    - [Introductory & Architectural](./docs/chainthru-index.md)
    - [Crate & functionality](...)

- ### HTTP-Server
    - [Introductory & Architectural](./docs/chainthru-api.md)
    - [Crate & functionality](...)

- ### Primitives
    - [Introductory & Architectural](./docs/chainthru-primitives.md)
    - [Crate & functionality](...)

- ### Chainthru (binary)
    - [Introductory & Architectural](./docs/chainthru.md)
    - [Crate](...)

- ### [Database](./docs/database.md)
- ### [Deployment (Helm)](./docs/deploy-helm.md)
