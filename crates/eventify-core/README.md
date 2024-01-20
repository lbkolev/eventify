## eventify-core

> <p align="center"> 🚧 work in progress 🚧<p>

```mermaid
graph TD
    Manager --> Collector
    Collector --> NodeClient
    Collector --> StorageClient
    NodeClient --> Ethereum
    NodeClient --> Starkware
    NodeClient --> Zksync
    StorageClient --> Postgres
```
