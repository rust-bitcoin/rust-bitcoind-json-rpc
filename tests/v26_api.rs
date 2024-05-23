//! Test the JSON-RPC API against `bitcoind v26.0`.

#![cfg(feature = "client-sync")]

mod client;

use bitcoind_json_rpc::client_sync::v26::Client;
// use bitcoind_json_rpc::model;

/// The `bitcoind` RPC port this test client should connect to.
const RPC_PORT: u16 = 26149;

client::impl_constructors!();

// == Blockchain ==
mod blockchain {
    use super::*;

    crate::impl_test_v17__getblockchaininfo!();
    crate::impl_test_v17__getbestblockhash!();
    crate::impl_test_v22__getblock!();
    //    crate::impl_test_v17__gettxout!();}
}

// == Control ==
mod control {
    // use super::*;

    // crate::impl_test_v17__stop!();
}

// == Network ==
mod network {
    use super::*;

    crate::impl_test_v17__getnetworkinfo!();
}

// == Wallet ==
mod wallet {
    use super::*;

    crate::impl_test_v17__createwallet!();
    // FIXME: Broken
    // crate::impl_test_v22__unloadwallet!();

    crate::impl_test_v17__getnewaddress!();
}
