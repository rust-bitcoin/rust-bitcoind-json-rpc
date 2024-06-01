// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Requires `Client` to be in scope.
//!
//! Specifically this is methods found under the `== Network ==` section of the
//! API docs of `bitcoind v0.17.1`.
//!
//! See, or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements bitcoind JSON-RPC API method `getnetworkinfo`
#[macro_export]
macro_rules! impl_client_v17__getnetworkinfo {
    () => {
        impl Client {
            /// Checks that the JSON-RPC endpoint is for a `bitcoind` instance with the expected version.
            pub fn check_expected_server_version(&self) -> Result<()> {
                let server_version = self.server_version()?;
                if server_version != EXPECTED_SERVER_VERSION {
                    return Err(UnexpectedServerVersionError {
                        got: server_version,
                        expected: EXPECTED_SERVER_VERSION,
                    })?;
                }
                Ok(())
            }

            /// Returns the server version field of `GetNetworkInfo`.
            pub fn server_version(&self) -> Result<usize> {
                let info = self.get_network_info()?;
                Ok(info.version)
            }

            pub fn get_network_info(&self) -> Result<GetNetworkInfo> {
                self.call("getnetworkinfo", &[])
            }
        }
    };
}
