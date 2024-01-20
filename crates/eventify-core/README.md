## eventify-core

> <p align="center"> 🚧 work in progress 🚧<p>

```mermaid
graph TD
    Manager --> Collector
    Collector --> NodeProvider
    Collector --> StorageClient
    NodeProvider --> Ethereum
    NodeProvider --> Starkware
    NodeProvider --> Zksync
    StorageClient --> Postgres
```
