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

> <p align="center"> 🚧 work in progress 🚧<p>

Eventify is an event listener and indexer designed with the following objectives in mind:
- Implementing an efficient event propagation mechanism.
- Offering a user-friendly means to index blocks, transactions, and events.

## Crates
Include the following:
- [eventify](./crates/eventify/) - Provides a CLI implementation for the event *listener*.
- [eventify-core](./crates/eventify-core) - Holds L1/L2 event listener, propagator & indexer.
- [eventify-http-server](./crates/eventify-http-server) - Exposes an HTTP server for the indexed data.
- [eventify-http-client](.crates//eventify-http-client) - Provides an OpenAPI generated client for the exposed API.
- [eventify-primitives](./crates/eventify-primitives) - Contains types shared between the different workspace crates.
