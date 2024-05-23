// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing test methods on a JSON-RPC client.
//!
//! Specifically this is methods found under the `== Generating ==` section of the
//! API docs of `bitcoind v0.17.1`.

/// Requires `Client` to be in scope and to implement `get_blockchain_info`.
#[macro_export]
macro_rules! impl_test_v17__generatetoaddress {
    () => {
        #[test]
        fn generate_to_address() {
            // Use one client to create the new wallet.
            let client = client();
            let wallet = format!("wallet-{}", rand::random::<u32>()).to_string();
            let _ = client.create_wallet(&wallet).expect("createwallet <random-wallet>");

            // And another for the wallet path calls.
            let wallet_client = client_for_wallet(&wallet);
            let address = wallet_client.new_address().expect("getnewaddress");

            let _ = client.generate_to_address(1, &address).expect("generatetoaddress");
        }
    };
}
