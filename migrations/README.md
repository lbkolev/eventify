## Database

The Database is referenced by both [The indexer](../eventify-idx/), as well as the [HTTP-Server](../eventify-http-server/).

Generally, there are two different ways of using `eventify`

- with both Indexer & Server running simultaneously on different threads.
- with either `eventify-idx` or `eventify-http-server` one of them running (e.g it might be desirable to decouple responsibilities in case the server receives high traffic, so as to avoid unnecessary service interruptions/slowdowns).

## Postgres - schema

```mermaid
erDiagram

"eth.block" {
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

"eth.transaction" {
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

"eth.log" {
    SERIAL id "PK"
    BYTEA address
    BYTEA topic0
    BYTEA topic1
    BYTEA topic2
    BYTEA topic3
    BYTEA data
    BYTEA block_hash
    BIGSERIAL block_number
    BYTEA transaction_hash
    NUMBER transaction_index
    BYTEA transaction_log_index
    BYTEA log_index
    TEXT log_type
    BOOL removed
}

"eth.contract" {
    BYTEA transaction_hash
    BYTEA _from
    BYTEA input
    TIMESTAMP created_at "TS of the table entry"
}

"eth.function_signature" {
    uuid id "PK"
    BYTEA hex_sig
    BYTEA text_sig
}
```
