## eventify-idx

> <p align="center"> 🚧 work in progress 🚧<p>

```mermaid
graph TD
    Manager --> Collector
    Collector --> NodeProvider
    Collector --> StorageProvider

    NodeProvider --> Ethereum
    NodeProvider --> Starkware
    NodeProvider --> Zksync

    StorageProvider --> Postgres
```
