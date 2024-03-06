# <p align="center">Eventify</p>

<p align="center">
<a href="https://github.com/lbkolev/eventify/blob/master/LICENSE-MIT">
<img src="https://img.shields.io/badge/license-MIT-blue.svg">
</a>
<a href="https://github.com/lbkolev/eventify/blob/master/LICENSE-APACHE">
<img src="https://img.shields.io/badge/license-APACHE2.0-blue.svg">
</a>
<a href="https://crates.io/crates/eventify">
<img src="https://img.shields.io/crates/v/eventify-cli.svg">
</a>
<a href="https://github.com/lbkolev/eventify/actions?query=workflow%3ACI+branch%3Amaster">
<img src="https://github.com/lbkolev/eventify/actions/workflows/ci.yml/badge.svg">
</a>
<a href="https://docs.rs/eventify">
<img src="https://img.shields.io/docsrs/eventify/latest">
</a>
</p>

## High level overview
<p align="center">
  <img src=".github/hl-overview-diagram.png" alt="Eventify High-Level Overview Diagram" width="850">
</p>

Eventify is an event pipeline designed with the following objectives in mind:
- Implementing an efficient event propagation mechanism.
- Offering a robust and user-friendly means to consume live data.

## Implemented ledgers
- Ethereum

## Example usage
*example configurations can be found at etc/configs/*
### Bring up the necessary services
```sh
docker-compose up -d postgres redis
```

### Run eventify
```sh
RUST_LOG=INFO cargo r -- run --config etc/configs/stream-eth-all.toml
```
```sh
RUST_LOG=INFO cargo r -- run --collect=blocks --network=eth \
  --node-url="wss://eth.llamarpc.com" \
  --database-url="postgres://postgres:password@localhost:5432/eventify" \
  --queue-url="redis://localhost:6379"
```

## Crates
Include the following:
- [eventify](./crates/eventify/) - Provides a CLI implementation that propagates the events to redis queues.
- [eventify-core](./crates/eventify-core) - Holds the pipeline & clients implementation.
- [eventify-http-server](./crates/eventify-http-server) - Exposes an HTTP server for the collected data. [wip 🚧]
- [eventify-primitives](./crates/eventify-primitives) - Contains generic & concrete types for use by anything using to stream live data.
- [eventify-configs](./crates/eventify-configs) - Keeps configuration for some of the core types used throughout the event pipeline.
