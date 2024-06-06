// SPDX-License-Identifier: CC0-1.0

//! A JSON-RPC client for testing against Bitcoin Core `v0.21.2`.
//!
//! We ignore option arguments unless they effect the shape of the returned JSON data.

use bitcoin::address::{Address, NetworkChecked};
use bitcoin::{Amount, Block, BlockHash, Txid};

use crate::client_sync::{handle_defaults, into_json};
use crate::json::v21::*;

crate::define_jsonrpc_minreq_client!("v21");

// == Blockchain ==
crate::impl_client_v17__getblockchaininfo!();
crate::impl_client_v17__getbestblockhash!();
crate::impl_client_v17__getblock!();
crate::impl_client_v17__gettxout!();

// == Network ==
crate::impl_client_v17__getnetworkinfo!();
crate::impl_client_check_expected_server_version!({ [210200] });

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
crate::impl_client_v19__getbalances!();
crate::impl_client_v17__sendtoaddress!();
