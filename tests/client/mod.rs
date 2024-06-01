//! Provides a macro that implements the tests.

pub mod v17;
pub mod v22;

/// Requires `RPC_PORT` to be in scope.
macro_rules! impl_constructors {
    () => {
        use bitcoind::BitcoinD;

        /// Initialize a logger (configure with `RUST_LOG=trace cargo test`).
        #[allow(dead_code)] // Not all tests use this function.
        fn init_logger() { let _ = env_logger::try_init(); }

        /// Returns a handle to a `bitcoind` instance with "default" wallet loaded.
        #[allow(dead_code)] // Not all tests use this function.
        fn bitcoind_with_default_wallet() -> BitcoinD {
            let exe = bitcoind::exe_path().expect("failed to get bitcoind executable");

            let conf = bitcoind::Conf::default();
            BitcoinD::with_conf(exe, &conf).expect("failed to create BitcoinD")
        }

        /// Returns a handle to a `bitcoind` instance without any wallets.
        #[allow(dead_code)] // Not all tests use this function.
        fn bitcoind_with_wallet(wallet: String) -> BitcoinD {
            let exe = bitcoind::exe_path().expect("failed to get bitcoind executable");

            let mut conf = bitcoind::Conf::default();
            conf.wallet = Some(wallet);
            BitcoinD::with_conf(exe, &conf).expect("failed to create BitcoinD")
        }

        /// Returns a handle to a `bitcoind` instance without any wallet loaded.
        #[allow(dead_code)] // Not all tests use this function.
        fn bitcoind_no_wallet() -> BitcoinD {
            let exe = bitcoind::exe_path().expect("failed to get bitcoind executable");

            let mut conf = bitcoind::Conf::default();
            conf.wallet = None;
            BitcoinD::with_conf(exe, &conf).expect("failed to create BitcoinD")
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
