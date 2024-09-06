// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing test methods on a JSON-RPC client.
//!
//! Specifically this is methods found under the `== Control ==` section of the
//! API docs of `bitcoind v0.17.1`.

/// Requires `Client` to be in scope and to implement `getmemoryinfo`.
#[macro_export]
macro_rules! impl_test_v17__getmemoryinfo {
    () => {
        #[test]
        fn get_memory_info() {
            let bitcoind = $crate::bitcoind_no_wallet();
            // There is no model for `getmemoryinfo`, just check we can make the call.
            let _ = bitcoind.client.get_memory_info().expect("getmemoryinfo");
        }
    };
}

/// Requires `Client` to be in scope and to implement `logging`.
#[macro_export]
macro_rules! impl_test_v17__logging {
    () => {
        #[test]
        fn logging() {
            let bitcoind = $crate::bitcoind_no_wallet();
            // There is no model for `logging`, just check we can make the call.
            let _ = bitcoind.client.logging().expect("logging");
        }
    };
}

/// Requires `Client` to be in scope and to implement `stop`.
#[macro_export]
macro_rules! impl_test_v17__stop {
    () => {
        #[test]
        fn stop() {
            let bitcoind = $crate::bitcoind_no_wallet();
            // There is no json object for `stop`, we just return a string.
            let _ = bitcoind.client.stop().expect("stop");
        }
    };
}

/// Requires `Client` to be in scope and to implement `uptime`.
#[macro_export]
macro_rules! impl_test_v17__uptime {
    () => {
        #[test]
        fn uptime() {
            let bitcoind = $crate::bitcoind_no_wallet();
            // There is no json object for `stop`, we just return a int.
            let _ = bitcoind.client.uptime().expect("uptime");
        }
    };
}
