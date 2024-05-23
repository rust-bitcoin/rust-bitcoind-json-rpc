/// Requires `Client` to be in scope and to implement `createwallet`.
#[macro_export]
macro_rules! impl_test_v17__createwallet {
    () => {
        /// Creates a random wallet name.
        #[allow(dead_code)] // Not all tests need this.
        pub fn random_wallet_name() -> String {
            format!("wallet-{}", rand::random::<u32>()).to_string()
        }

        #[test]
        fn create_wallet() {
            let client = client();
            let wallet = random_wallet_name();

            let _ = client.create_wallet(&wallet).expect("createwallet <random-wallet>");
        }
    };
}

/// Requires `Client` to be in scope and to implement `unloadwallet`.
#[macro_export]
macro_rules! impl_test_v17__unloadwallet {
    () => {
        #[test]
        fn unload_wallet() {
            let client = client();
            let wallet = random_wallet_name();

            let _ = client.create_wallet(&wallet).expect("createwallet <random-wallet>");
            let _ = client.unload_wallet(&wallet).expect("unloadwallet <random-wallet>");
        }
    };
}

/// Requires `Client` to be in scope and to implement `get_new_address`.
#[macro_export]
macro_rules! impl_test_v17__getnewaddress {
    () => {
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
        #[allow(dead_code)] // Not all tests need this.
        pub fn wait() {
            let wait = std::time::Duration::from_millis(100);
            std::thread::sleep(wait);
        }

        #[test]
        fn get_new_address() { let (_, _, _) = setup(); }
    };
}

/// Requires `Client` to be in scope and to implement `get_balance`.
#[macro_export]
macro_rules! impl_test_v17__getbalance {
    () => {
        #[test]
        fn get_balance() {
            let (client, wallet_client, address) = setup();
            let _ = client.generate_to_address(101, &address).expect("generatetoaddress");
            wait();

            let _ = wallet_client.get_balance().expect("getbalance");
        }
    };
}

/// Requires `Client` to be in scope and to implement `send_to_address`.
#[macro_export]
macro_rules! impl_test_v17__sendtoaddress {
    () => {
        #[test]
        fn send_to_address() {
            use bitcoin::Amount;

            let (client, wallet_client, address) = setup();
            let _ = client.generate_to_address(110, &address).expect("generatetoaddress");
            wait();

            let balance = wallet_client.get_balance().expect("getbalance");
            dbg!(balance);

            let new = wallet_client.new_address().expect("failed to get a new address");
            let _ = wallet_client
                .send_to_address(&new, Amount::from_sat(10_000))
                .expect("sendtoaddress");
        }
    };
}
