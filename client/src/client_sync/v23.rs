// SPDX-License-Identifier: CC0-1.0

//! A JSON-RPC client for testing against Bitcoin Core `v23`.
//!
//! We ignore option arguments unless they effect the shape of the returned JSON data.

use bitcoin::address::{Address, NetworkChecked};
use bitcoin::{Amount, Block, BlockHash, Txid};
use serde::{Deserialize, Serialize};

use crate::client_sync::{handle_defaults, into_json};
use crate::json::v23::*;

crate::define_jsonrpc_minreq_client!("v23");

// == Blockchain ==
crate::impl_client_v17__getblockchaininfo!();
crate::impl_client_v17__getbestblockhash!();
crate::impl_client_v17__getblock!();
crate::impl_client_v17__gettxout!();

// == Control ==
crate::impl_client_v17__stop!();

// == Generating ==
crate::impl_client_v17__generatetoaddress!();

// == Network ==
crate::impl_client_v17__getnetworkinfo!();
crate::impl_client_check_expected_server_version!({ [230000, 230100, 230200] });

// == Rawtransactions ==
crate::impl_client_v17__sendrawtransaction!();

// == Wallet ==
crate::impl_client_v17__createwallet!();
crate::impl_client_v22__unloadwallet!();
crate::impl_client_v22__loadwallet!();
crate::impl_client_v17__getbalance!();
crate::impl_client_v19__getbalances!();
crate::impl_client_v17__getnewaddress!();
crate::impl_client_v17__sendtoaddress!();
crate::impl_client_v17__gettransaction!();

/// Argument to the `Client::get_new_address_with_type` function.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AddressType {
    Legacy,
    P2ShSegwit,
    Bech32,
    Bech32m,
}

impl fmt::Display for AddressType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AddressType::*;

        let s = match *self {
            Legacy => "legacy",
            P2ShSegwit => "p2sh-segwit",
            Bech32 => "bech32",
            Bech32m => "bech32m",
        };
        fmt::Display::fmt(s, f)
    }
}
