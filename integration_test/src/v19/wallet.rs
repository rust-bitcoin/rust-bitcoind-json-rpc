/// Requires `Client` to be in scope and to implement `get_balances`.
#[macro_export]
macro_rules! impl_test_v19__getbalances {
    () => {
        #[test]
        fn get_balances() {
            let bitcoind = $crate::bitcoind_with_default_wallet();
            let address = bitcoind.client.new_address().expect("failed to get new address");
            let _ = bitcoind.client.generate_to_address(101, &address).expect("generatetoaddress");
            let json = bitcoind.client.get_balances().expect("getbalances");
            json.into_model().unwrap();
        }
    };
}
