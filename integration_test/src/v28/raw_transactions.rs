// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing test methods on a JSON-RPC client.
//!
//! Specifically this is methods found under the `== Rawtransactions ==` section of the
//! API docs of `bitcoind v28.0`.

/// Requires `Client` to be in scope
#[macro_export]
macro_rules! impl_test_v28__submitpackage {
    () => {
        #[test]
        fn submitpackage() {
            //let bitcoind = $crate::bitcoind_no_wallet();

            let bitcoind = $crate::bitcoind_with_default_wallet();

            // Submitting the empty package should simply fail.
            assert!(bitcoind.client.submit_package(&[], None, None).is_err());

            // Premine to get some funds
            let address = bitcoind.client.new_address().expect("failed to get new address");
            let json =
                bitcoind.client.generate_to_address(101, &address).expect("generatetoaddress");
            json.into_model().unwrap();

            // Send to ourselves, mine, send again to generate two transactions.
            let (tx_0, tx_1) = {
                let new_address = bitcoind.client.new_address().expect("failed to get new address");
                let txid = bitcoind
                    .client
                    .send_to_address(&new_address, bitcoin::Amount::from_sat(1000000))
                    .unwrap()
                    .into_model()
                    .unwrap()
                    .txid;

                let _ =
                    bitcoind.client.generate_to_address(1, &address).expect("generatetoaddress");

                let best_block_hash = bitcoind.client.best_block_hash().unwrap();
                let best_block = bitcoind.client.get_block(best_block_hash).unwrap();
                let tx_0 = best_block.txdata[1].clone();

                let new_address = bitcoind.client.new_address().expect("failed to get new address");
                let txid = bitcoind
                    .client
                    .send_to_address(&new_address, bitcoin::Amount::from_sat(1000000))
                    .unwrap()
                    .into_model()
                    .unwrap()
                    .txid;

                let _ =
                    bitcoind.client.generate_to_address(1, &address).expect("generatetoaddress");

                let best_block_hash = bitcoind.client.best_block_hash().unwrap();
                let best_block = bitcoind.client.get_block(best_block_hash).unwrap();
                let tx_1 = best_block.txdata[1].clone();
                (tx_0, tx_1)
            };

            // The call for submitting this package should succeed, but yield an 'already known'
            // error for all transactions.
            let res = bitcoind
                .client
                .submit_package(&[tx_0, tx_1], None, None)
                .expect("failed to submit package");
            for (_, tx_result) in &res.tx_results {
                assert!(tx_result.error.is_some());
            }
            assert!(res.replaced_transactions.is_empty());
        }
    };
}
