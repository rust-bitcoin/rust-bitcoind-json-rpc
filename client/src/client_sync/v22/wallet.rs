// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of `bitcoind v22.1`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements bitcoind JSON-RPC API method `unloadwallet`
#[macro_export]
macro_rules! impl_client_v22__unloadwallet {
    () => {
        impl Client {
            pub fn unload_wallet(&self, wallet: &str) -> Result<UnloadWallet> {
                self.call("unloadwallet", &[wallet.into()])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `loadwallet`
#[macro_export]
macro_rules! impl_client_v22__loadwallet {
    () => {
        impl Client {
            pub fn load_wallet(&self, wallet: &str) -> Result<LoadWallet> {
                self.call("loadwallet", &[wallet.into()])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getbalances`
#[macro_export]
macro_rules! impl_client_v22__getbalances {
    () => {
        impl Client {
            pub fn get_balances(&self) -> Result<GetBalances> { self.call("getbalances", &[]) }
        }
    };
}
