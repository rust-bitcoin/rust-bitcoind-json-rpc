// SPDX-License-Identifier: CC0-1.0

//! A JSON-RPC client for testing against Bitcoin Core `v0.17.1`.
//!
//! We ignore option arguments unless they effect the shape of the returned JSON data.

pub mod blockchain;
pub mod control;
pub mod generating;
pub mod network;
pub mod wallet;

use bitcoin::address::{Address, NetworkChecked};
use bitcoin::{Amount, Block, BlockHash, Txid};

use crate::client_sync::{handle_defaults, into_json};
use crate::json::v17::*;

/// Support Bitcoin Core `v0.17.1`.
const EXPECTED_SERVER_VERSION: usize = 170100;

crate::define_jsonrpc_minreq_client!();
//crate::impl_client_helpers!();

// == Network ==
crate::impl_client_v17__getnetworkinfo!();

// == Blockchain ==
crate::impl_client_v17__getblockchaininfo!();
crate::impl_client_v17__getbestblockhash!();
crate::impl_client_v17__getblock!();
crate::impl_client_v17__gettxout!();

// == Control ==
crate::impl_client_v17__stop!();

// == Generating ==
crate::impl_client_v17__generatetoaddress!();

// == Wallet ==
crate::impl_client_v17__createwallet!();
crate::impl_client_v17__unloadwallet!();
crate::impl_client_v17__loadwallet!();
crate::impl_client_v17__getnewaddress!();
crate::impl_client_v17__getbalance!();
crate::impl_client_v17__sendtoaddress!();

//crate::impl_client_v17_generatetoaddress!();

// #[macro_export]
// macro_rules! impl_client_v17_generatetoaddress {
//     () => {
//         impl Client {
//             pub fn generate_to_address(
//                 &self,
//                 block_num: u64,
//                 address: &Address<NetworkChecked>,
//             ) -> Result<GenerateToAddress> {
//                 self.call("generatetoaddress", &[block_num.into(), address.to_string().into()])
//             }
//         }
//     }
// }

// #[macro_export]
// macro_rules! impl_client_v17_api {
//     () => {
//         impl Client {
//             pub fn get_balance(&self) -> Result<GetBalance> { self.call("getbalance", &[]) }
//         }
//     };
// }

// SPDX-License-Identifier: CC0-1.0
