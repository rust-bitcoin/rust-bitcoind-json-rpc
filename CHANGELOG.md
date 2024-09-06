# unreleased

- Add support for Bitcoin Core versions `26.1`, `26.2`, `27.0`, and `27.1` [#21](https://github.com/rust-bitcoin/rust-bitcoind-json-rpc/pull/21)
- Import `jsonrpc` crate [#19](https://github.com/rust-bitcoin/rust-bitcoind-json-rpc/pull/19)
- Bump MSRV to Rust `v1.63.0` [#17](https://github.com/rust-bitcoin/rust-bitcoind-json-rpc/pull/17)
- Migrate repo to `github.com/rust-bitcoin` org (from `github.com/tcharding`).

# 0.3 - June 2024

- Switch from implementing `TryFrom` to implementing inherent `into_model` function [#9](https://github.com/rust-bitcoin/rust-bitcoind-json-rpc/pull/9)

# 0.2 - June 2024

The first two versions were what was needed to get integration testing
in `rust-miniscript` to be able to use this crate.