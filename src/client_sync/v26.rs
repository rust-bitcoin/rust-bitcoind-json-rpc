// SPDX-License-Identifier: CC0-1.0

//! A JSON-RPC client for testing against Bitcoin Core `v26.0`.
//!
//! We ignore option arguments unless they effect the shape of the returned JSON data.

use bitcoin::{Block, BlockHash, Txid};

use crate::client_sync::into_json;
use crate::json::v26::*;

/// Support Bitcoin Core `v26.0`.
const EXPECTED_SERVER_VERSION: usize = 260100;

crate::define_jsonrpc_minreq_client!();

// == Blockchain ==
crate::impl_client_v17__getblockchaininfo!();
crate::impl_client_v17__getbestblockhash!();
crate::impl_client_v17__getblock!();
crate::impl_client_v17__gettxout!();

// == Control ==
crate::impl_client_v17__stop!();

// == Network ==
crate::impl_client_v17__getnetworkinfo!();

// == Wallet ==
crate::impl_client_v17__createwallet!();
crate::impl_client_v17__unloadwallet!();
crate::impl_client_v17__loadwallet!();
// FIXME: Currently this uses generatetoaddress
// crate::impl_client_v17__getbalance!();
crate::impl_client_v17__getnewaddress!();
// FIXME: Currently this uses generatetoaddress
// crate::impl_client_v17__sendtoaddress!();
