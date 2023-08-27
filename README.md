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
    - [Index](#index)
    - [Server](#server)
    - [Primitives](#primitives)
    - [Tracing](#tracing)
    - [Core](#core)


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
    NUMBER number
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

"contract_fn.transfer" ||--o{ "public.contract": "depends on"
"contract_fn.transfer" {
    bytea contract_addr "FK"
    bytea transaction_hash "PK"
    bytea transaction_sender
    bytea _to
    bytea _value
    TIMESTAMP created_at "The TS of the table entry"
}

"contract_fn.transfer_from" ||--o{ "public.contract": "depends on"
"contract_fn.transfer_from" {
    bytea contract_addr "FK"
    bytea transaction_hash "PK"
    bytea transaction_sender
    bytea _from
    bytea _to
    bytea _value
    TIMESTAMP created_at "The TS of the table entry"
}

"contract_fn.approve" ||--o{ "public.contract": "depends on"
"contract_fn.approve" {
    bytea contract_addr "FK"
    bytea transaction_hash "PK"
    bytea transaction_sender
    bytea _spender
    bytea _value
    TIMESTAMP created_at "The TS of the table entry"
}

"contract_fn.safe_transfer_from" ||--o{ "public.contract": "depends on"
"contract_fn.safe_transfer_from" {
    bytea contract_addr "FK"
    bytea transaction_hash "PK"
    bytea transaction_sender
    bytea _from
    bytea _to
    bytea _token_id
    TIMESTAMP created_at "The TS of the table entry"
}

"public.function_signature" {
    uuid id "PK"
    bytea hex_sig
    bytea text_sig
}
```

## Crates
Include the following
- [chainthru (binary)](./chainthru/)
- [chainthru-index](./chainthru-index)
- [chainthru-server](./chainthru-server)
- [chainthru-primitives](./chainthru-primitives)
- [chainthru-tracing](./chainthru-tracing)

## Documentation

### Index
- [Introductory & Architectural](./docs/chainthru-index.md)
- [Crate & functionality](...)

### Server
- [Introductory & Architectural](./docs/chainthru-api.md)
- [Crate & functionality](...)

### Primitives
- [Introductory & Architectural](./docs/chainthru-primitives.md)
- [Crate & functionality](...)

### Tracing
- [Introductory & Architectural](./docs/chainthru-tracing.md)
- [Crate & functionality](...)

### Core
- [Introductory & Architectural](./docs/chainthru.md)
- [Crate & functionality](...)