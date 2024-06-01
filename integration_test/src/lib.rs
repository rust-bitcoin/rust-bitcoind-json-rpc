//! Provides a macro that implements the tests.

pub mod v17;
pub mod v22;

/// Requires `RPC_PORT` to be in scope.
use bitcoind::BitcoinD;

/// Initialize a logger (configure with `RUST_LOG=trace cargo test`).
#[allow(dead_code)] // Not all tests use this function.
pub fn init_logger() { let _ = env_logger::try_init(); }

/// Returns a handle to a `bitcoind` instance with "default" wallet loaded.
#[allow(dead_code)] // Not all tests use this function.
pub fn bitcoind_with_default_wallet() -> BitcoinD {
    let exe = bitcoind::exe_path().expect("failed to get bitcoind executable");

    let conf = bitcoind::Conf::default();
    BitcoinD::with_conf(exe, &conf).expect("failed to create BitcoinD")
}

/// Returns a handle to a `bitcoind` instance without any wallets.
#[allow(dead_code)] // Not all tests use this function.
pub fn bitcoind_with_wallet(wallet: String) -> BitcoinD {
    let exe = bitcoind::exe_path().expect("failed to get bitcoind executable");

    let mut conf = bitcoind::Conf::default();
    conf.wallet = Some(wallet);
    BitcoinD::with_conf(exe, &conf).expect("failed to create BitcoinD")
}

/// Returns a handle to a `bitcoind` instance without any wallet loaded.
#[allow(dead_code)] // Not all tests use this function.
pub fn bitcoind_no_wallet() -> BitcoinD {
    let exe = bitcoind::exe_path().expect("failed to get bitcoind executable");

    let mut conf = bitcoind::Conf::default();
    conf.wallet = None;
    BitcoinD::with_conf(exe, &conf).expect("failed to create BitcoinD")
}
