// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of `bitcoind v0.19.1`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements bitcoind JSON-RPC API method `getbalances`
#[macro_export]
macro_rules! impl_client_v19__getbalances {
    () => {
        impl Client {
            pub fn get_balances(&self) -> Result<GetBalances> { self.call("getbalances", &[]) }
        }
    };
}
