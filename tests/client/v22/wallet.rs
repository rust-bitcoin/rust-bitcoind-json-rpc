/// Requires `Client` to be in scope and to implement `unloadwallet`.
#[macro_export]
macro_rules! impl_test_v22__unloadwallet {
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

/// Requires `Client` to be in scope and to implement `get_balances`.
#[macro_export]
macro_rules! impl_test_v22__getbalances {
    () => {
        #[test]
        fn get_balances() {
            let (client, wallet_client, address) = setup();
            let _ = client.generate_to_address(101, &address).expect("generatetoaddress");
            wait();

            let _ = wallet_client.get_balances().expect("getbalances");
        }
    };
}
