// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Control ==` section of the
//! API docs of `bitcoind v0.17.1`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements bitcoind JSON-RPC API method `stop`
#[macro_export]
macro_rules! impl_client_v17__stop {
    () => {
        impl Client {
            pub fn stop(&self) -> Result<String> { self.call("stop", &[]) }
        }
    };
}
