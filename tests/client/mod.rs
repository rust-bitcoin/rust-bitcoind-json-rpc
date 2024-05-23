//! Provides a macro that implements the tests.

pub mod v17;
pub mod v22;

/// Requires `RPC_PORT` to be in scope.
macro_rules! impl_constructors {
    () => {
        /// Creates a `Client` using `http:://localhost:RPC_PORT/`.
        #[allow(dead_code)] // Not all tests need this.
        fn client() -> Client {
            let url = format!("http://localhost:{}", RPC_PORT);
            client_with_url(&url)
        }

        /// Creates a `Client` using `http:://localhost:RPC_PORT/wallet/<wallet>`.
        #[allow(dead_code)] // Not all tests need this.
        fn client_for_wallet(wallet: &str) -> Client {
            let url = format!("http://localhost:{}/wallet/{}", RPC_PORT, wallet);
            client_with_url(&url)
        }

        /// Creates a `Client` using `url`.
        #[allow(dead_code)] // Not all tests need this.
        fn client_with_url(url: &str) -> Client {
            // See ../run-local-core-nodes.sh
            const RPC_USER: &str = "user";
            const RPC_PASSWORD: &str = "password";

            let user = RPC_USER.to_string();
            let pass = Some(RPC_PASSWORD.to_string());
            let client = Client::new_with_auth(url, user, pass);

            client.check_expected_server_version().expect("correct server version");
            client
        }
    };
}
pub(crate) use impl_constructors;

/// Requires the correct `Client` to be in scope.
///
/// Requires the generatetoaddress API method.
#[macro_export]
macro_rules! impl_extended_tests_wallet {
    ($mod_name:ident) => {
        pub use $mod_name::{setup, wait};

        mod $mod_name {
            use std::{thread, time};

            use super::*;

            pub fn setup() -> (Client, Client, bitcoin::Address) {
                // Use one client to create the new wallet.
                let client = client();
                let wallet = random_wallet_name();
                let _ = client.create_wallet(&wallet).expect("createwallet <random-wallet>");

                // And another for the wallet path calls.
                let wallet_client = client_for_wallet(&wallet);
                let address = wallet_client.new_address().expect("getnewaddress");

                (client, wallet_client, address)
            }

            // For flaky tests, just wait a bit.
            pub fn wait() {
                let wait = time::Duration::from_millis(100);
                thread::sleep(wait);
            }

            #[test]
            fn new_address() { let _ = setup(); }

            #[test]
            fn generate_to_address() {
                let (client, _, address) = setup();
                let _ = client.generate_to_address(101, &address).expect("generatetoaddress");
            }

            #[test]
            fn get_balance() {
                let (client, wallet_client, address) = setup();
                let _ = client.generate_to_address(101, &address).expect("generatetoaddress");
                wait();

                let _ = wallet_client.get_balance().expect("getbalance");
            }

            #[test]
            fn send_to_address() {
                use bitcoin::Amount;

                let (client, wallet_client, address) = setup();
                let _ = client.generate_to_address(1000, &address).expect("generatetoaddress");
                wait();

                let balance = wallet_client.get_balance().expect("getbalance");
                dbg!(balance);

                let new = wallet_client.new_address().expect("failed to get a new address");
                let _ = wallet_client
                    .send_to_address(&new, Amount::from_sat(10_000))
                    .expect("sendtoaddress");
            }
        }
    };
}

/// Requires the correct `Client` to be in scope.
///
/// Requires the `impl_extended_tests_wallet` macro to have been called.
#[macro_export]
macro_rules! impl_test_wallet_balances {
    ($mod_name:ident) => {
        mod $mod_name {
            use super::*;

            #[test]
            fn get_balances() {
                let (client, wallet_client, address) = setup();
                let _ = client.generate_to_address(101, &address).expect("generatetoaddress");
                wait();

                let _ = wallet_client.get_balances().expect("getbalances");
            }
        }
    };
}
