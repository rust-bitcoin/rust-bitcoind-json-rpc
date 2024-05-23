# Bitcoin Core JSON-RPC support

Much of the code in this crate was shamelessy stolen from [rust-bitcoincore-rpc
v0.19.0](https://github.com/rust-bitcoin/rust-bitcoincore-rpc). As such I have maintained the
original author's list and added my own name for accountability. All bugs are my own (tcharding).

This is a demo of what may be needed to achieve the following aim:

1. Provide JSON types for some set of Bitcoin Core versions.
2. Provide a sync client that can be used for integration testing.
3. Extensively test RPC methods (excl. optional args) for each version of Core we claim to support.

## Client conundrum

- It seems reasonable that `rust-bitcoin` should provide the JSON-RPC types for Bitcoin Core.
- It seems unreasonable that `rust-bitcoin` should provide a JSON-RPC client.
- However in order to test the types we have to implement a client.

## Status

- Bitcoin Core `v0.17.1`, `v22.1`, and `v26.1`.
- RPC methods
   - `getblockchaininfo`
   - `getbestblockhash`
   - `getblock 0`
   - `getblock 1`
   - `gettxout`                    <!-- Untested -->
   - `getnetworkinfo`
   - `createwallet`
   - `loadwallet`
   - `unloadwallet`
   - `getbalance`
   - `getbalances`
   - `getnewaddress`
   - `generatetoaddress`
   - `sendtoaddress`
   - `stop`
