# <p align="center">eventify</p>
<p align="center"> Onchain Indexer / Event listener.</p>

<p align="center">
    <a href="https://github.com/lbkolev/eventify/blob/master/LICENSE-MIT">
        <img src="https://img.shields.io/badge/license-MIT-blue.svg">
    </a>
    <a href="https://github.com/lbkolev/eventify/blob/master/LICENSE-APACHE">
        <img src="https://img.shields.io/badge/license-APACHE2.0-blue.svg">
    </a>
    <a href="https://crates.io/crates/eventify">
        <img src="https://img.shields.io/crates/v/eventify.svg">
    </a>
    <a href="https://github.com/lbkolev/eventify/actions?query=workflow%3ACI+branch%3Amaster">
        <img src="https://github.com/lbkolev/eventify/actions/workflows/ci.yml/badge.svg">
    </a>
    <a href="https://docs.rs/eventify">
        <img src="https://img.shields.io/docsrs/eventify/latest">
    </a>
</p>

## <p align="center"> 🚧 work in progress 🚧<p>
eventify is an Ethereum event listener and indexer designed with the following objectives in mind:
- Offering a user-friendly means to index Ethereum blocks, transactions, and events.
- Implementing an efficient event propagation mechanism.
- Providing a comprehensive interface for inspecting the Ethereum mempool.

Implemented storages:
- [x] Postgres

## Crates
Include the following:
- [eventify (binary)](./eventify/) - Provides a CLI interface implementation for the server/indexer.
- [eventify-idx](./eventify-idx) - Holds the Indexer implementation.
- [eventify-http-server](./eventify-http-server) - Exposes an HTTP server for the indexed data.
- [eventify-http-client](./eventify-http-client) - Provides an OpenAPI generated client for the exposed API.
- [eventify-middleware](./eventify-middleware) - Holds the traits/types necessary to implement a middleware interface.
- [eventify-primitives](./eventify-primitives) - Contains types shared between the different workspace crates.

## Documentation
- ### Indexer
    - [Introductory & Architectural](./crates/eventify-idx/README.md)
    - [Crate & functionality](https://crates.io/crates/eventify-idx)

- ### HTTP-Server
    - [Introductory & Architectural](./crates/eventify-http-server/README.md)
    - [Crate & functionality](https://crates.io/crates/eventify-http-server)

- ### Client
    - [Introductory & Architectural](./crates/eventify-http-client/README.md)
    - [Crate & functionality](https://crates.io/crates/eventify-http-client)

- ### Middleware
    - [Introductory & Architectural](./crates/eventify-middleware/README.md)
    - [Crate & functionality](https://crates.io/crates/eventify-middleware)

- ### Primitives
    - [Introductory & Architectural](./crates/eventify-primitives/README.md)
    - [Crate & functionality](https://crates.io/crates/eventify-primitives)

- ### eventify (binary)
    - [Introductory & Architectural](./crates/eventify/README.md)
    - [Crate](https://crates.io/crates/eventify)

- ### [Database](./docs/database.md)
- ### [Deployment (Helm)](./docs/deploy-helm.md)
