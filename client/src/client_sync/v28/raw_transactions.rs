// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Rawtransactions ==` section of the
//! API docs of `bitcoind v28.0`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements bitcoind JSON-RPC API method `submitpackage`
#[macro_export]
macro_rules! impl_client_v28__submitpackage {
    () => {
        impl Client {
            /// Submit a package of transactions to local node.
            ///
            /// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
            ///
            /// ## Arguments:
            /// 1. `package`:          An array of raw transactions.
            ///                     The package must solely consist of a child and its parents. None of the parents may depend on each other.
            ///                     The package must be topologically sorted, with the child being the last element in the array.
            /// 2. `maxfeerate`:       Reject transactions whose fee rate is higher than the specified value.
            ///                     Fee rates larger than 1BTC/kvB are rejected.
            ///                     Set to 0 to accept any fee rate.
            ///                     If unset, will default to 0.10 BTC/kvb.
            /// 3. `maxburnamount`    If set, reject transactions with provably unspendable outputs (e.g. 'datacarrier' outputs that use the OP_RETURN opcode) greater than the specified value.
            ///                     If burning funds through unspendable outputs is desired, increase this value.
            ///                     This check is based on heuristics and does not guarantee spendability of outputs.
            pub fn submit_package(
                &self,
                package: &[bitcoin::Transaction],
                max_fee_rate: Option<bitcoin::FeeRate>,
                max_burn_amount: Option<bitcoin::Amount>,
            ) -> Result<SubmitPackage> {
                let package_txs = package
                    .into_iter()
                    .map(|tx| bitcoin::consensus::encode::serialize_hex(tx))
                    .collect::<Vec<_>>();
                let max_fee_rate_btc_kvb =
                    max_fee_rate.map(|r| r.to_sat_per_vb_floor() as f64 / 100_000.0);
                let max_burn_amount_btc = max_burn_amount.map(|a| a.to_btc());
                self.call(
                    "submitpackage",
                    &[package_txs.into(), max_fee_rate_btc_kvb.into(), max_burn_amount_btc.into()],
                )
            }
        }
    };
}
