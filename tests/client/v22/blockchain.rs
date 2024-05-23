/// Requires `Client` to be in scope and to implement `get_block`.
#[macro_export]
macro_rules! impl_test_v22__getblock {
    () => {
        #[test]
        fn get_block() {
            use bitcoind_json_rpc::model;

            let client = client();
            let block_hash = best_block_hash();

            // Users who only want to use `json` module can do:
            // let block_hash = json.best_block_hash.parse::<BlockHash>()?;

            let json = client.get_block_verbosity_zero(&block_hash).expect("getblock 0");
            let _ = model::GetBlockVerbosityZero::try_from(json).unwrap();

            let json = client.get_block_verbosity_one(&block_hash).expect("getblock 1");
            let _ = model::GetBlockVerbosityOne::try_from(json).unwrap();

            // TODO: getblock 2
            // let json = client.get_block_verbosity_two(&block_hash).expect("getblock 2");
        }
    };
}
