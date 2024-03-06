## eventify-core

```mermaid
graph TD
    Manager --> Collector
    Collector --> Node
    Collector --> Storage
    Collector -> Queue
    Node --> Ethereum
    Node --> Zksync
    Storage --> Postgres
    Queue ->> Redis
```
