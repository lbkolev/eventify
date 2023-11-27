# <p align="center">chainthru</p>
<p align="center"> Index Ethereum into a Storage backend.</p>

<p align="center">
    <a href="https://github.com/lbkolev/chainthru/blob/master/LICENSE-MIT">
        <img src="https://img.shields.io/badge/license-MIT-blue.svg">
    </a>
    <a href="https://github.com/lbkolev/chainthru/blob/master/LICENSE-APACHE">
        <img src="https://img.shields.io/badge/license-APACHE2.0-blue.svg">
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

## <p align="center"> 🚧 work in progress 🚧<p>
Chainthru is an Ethereum ETL Indexer that aims to provide a seamless [and straightforward] way to index Ethereum blocks, transactions, and events.

Implemented storages:
- [x] Postgres

- [chainthru](#chainthru)
  - [Crates](#crates)
  - [Documentation](#documentation)

## Crates
Include the following
- [chainthru (binary)](./chainthru/) - Provides a CLI interface implementation for the server/indexer.
- [chainthru-index](./chainthru-index) - Holds the Indexer implementation.
- [chainthru-server](./chainthru-server) - Exposes an HTTP server for the indexed data.
- [chainthru-primitives](./chainthru-primitives) - Contains types shared between the different workspace crates.

## Documentation
- ### Indexer
    - [Introductory & Architectural](./docs/chainthru-index.md)
    - [Crate & functionality](https://crates.io/crates/chainthru-index)

- ### HTTP-Server
    - [Introductory & Architectural](./docs/chainthru-server.md)
    - [Crate & functionality](https://crates.io/crates/chainthru-server)

- ### Primitives
    - [Introductory & Architectural](./docs/chainthru-primitives.md)
    - [Crate & functionality](https://crates.io/crates/chainthru-primitives)

- ### Chainthru (binary)
    - [Introductory & Architectural](./docs/chainthru.md)
    - [Crate](...)

- ### [Database](./docs/database.md)
- ### [Deployment (Helm)](./docs/deploy-helm.md)
