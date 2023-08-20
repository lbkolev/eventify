## Database
Referenced by both [chainthru-index](../chainthru-index/) & [chainthru-server](../chainthru-server/).

Generally, there are two different ways of using `chainthru`
- with both Indexer & Server running on different threads. The migrations are idempotently ran by both crates.
- with either `chainthru-index` or `chainthru-server` running.