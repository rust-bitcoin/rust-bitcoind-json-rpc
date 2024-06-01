//! Test the JSON-RPC API against `bitcoind v26.0`.

// FIXME: bitcoind crate does not support this yet.
#![cfg(all(feature = "client-sync", feature = "v26"))]

mod client;

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
