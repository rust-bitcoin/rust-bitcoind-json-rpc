// SPDX-License-Identifier: CC0-1.0

//! A JSON-RPC client for testing against Bitcoin Core `v0.18.1`.
//!
//! We ignore option arguments unless they effect the shape of the returned JSON data.

use bitcoin::address::{Address, NetworkChecked};
use bitcoin::{Amount, Block, BlockHash, Txid};

use crate::client_sync::{handle_defaults, into_json};
use crate::json::v18::*;

/// Support Bitcoin Core `v0.18.1`.
const EXPECTED_SERVER_VERSION: usize = 180100;

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
