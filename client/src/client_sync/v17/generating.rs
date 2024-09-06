// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Generating ==` section of the
//! API docs of `bitcoind v0.17.1`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements bitcoind JSON-RPC API method `generatetoaddress`
#[macro_export]
macro_rules! impl_client_v17__generatetoaddress {
    () => {
        impl Client {
            pub fn generate_to_address(
                &self,
                nblocks: usize,
                address: &bitcoin::Address,
            ) -> Result<GenerateToAddress> {
                self.call("generatetoaddress", &[nblocks.into(), into_json(address)?])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `generate`
#[macro_export]
macro_rules! impl_client_v17__generate {
    () => {
        impl Client {
            pub fn generate(&self, nblocks: usize) -> Result<Generate> {
                self.call("generate", &[nblocks.into()])
            }
        }
    };
}
