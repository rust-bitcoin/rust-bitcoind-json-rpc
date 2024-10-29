# `bitcoind` JSON-RPC support

There are two primary purposes of this repository.

1. Provide the [`bitcoind-json-rpc-types`](https://crates.io/crates/bitcoind-json-rpc-types) crate
   for use in production software.
2. Provide tools to the community for integration testing code written in Rust that interacts with
   the Bitcoin network. Primarily consumers of the [`rust-bitcoin`](https://crates.io/crates/bitcoin)
   library. And enable doing so against multiple versions of Bitcoin Core.

If you require a JSON RPC client in production software it is expected you write your own and only
use the `bitcoind-json-rpc-types` crate in your dependency graph. Feel free to copy/steal/plagiarise
or otherwise enjoy yourself with anything in this repository - no attribution required.

**Please do not use `client` in production and raise bugs, issues, or feature requests.**

## Crate listing

- [`json`](https://crates.io/crates/bitcoind-json-rpc-types): Rust types returned by the JSON-RPC API of Bitcoin Core (`bitcoind-json-rpc-types`).
- [`regtest`](https://crates.io/crates/bitcoind-json-rpc-regtest): Runs `bitcoind` regtest nodes.
- [`client`](https://crates.io/crates/bitcoind-json-rpc-client): A blocking JSON-RPC client used to test `bitcoind-json-rpc-types`.
- `integration_test`: Integration tests that use `client` and `regtest` to test `json`.

## Original code

I don't know who is using `bitcoind` and/or `rust-bitocincore-rpc` in the wild and I do not want to
disrupt them. As such `bitcoind` was pulled in here with permission of the original author.

Some code shamelessly stolen from `rust-bitcoincore-rpc` (credit to Steven).

- [rust-bitcoincore-rpcv0.19.0](https://github.com/rust-bitcoin/rust-bitcoincore-rpc)
- [`bitcoind`](https://crates.io/crates/bitcoind)

## Minimum Supported Rust Version (MSRV)

This library should always compile with any combination of features on **Rust 1.63.0**.

Use `Cargo-minimal.lock` to build the MSRV by copying to `Cargo.lock` and building.
