// All features uses 26_0
#[cfg(feature = "26_0")]
#[allow(unused_imports)] // Not all users need the json types.
pub use bitcoind_json_rpc_client::{client_sync::v26::Client, json::v26 as json};

#[cfg(all(feature = "25_2", not(feature = "26_0")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use bitcoind_json_rpc_client::{client_sync::v25::Client, json::v25 as json};

#[cfg(all(feature = "25_1", not(feature = "25_2")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use bitcoind_json_rpc_client::{client_sync::v25::Client, json::v25 as json};

#[cfg(all(feature = "25_0", not(feature = "25_1")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use bitcoind_json_rpc_client::{client_sync::v25::Client, json::v25 as json};

#[cfg(all(feature = "24_2", not(feature = "25_0")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use bitcoind_json_rpc_client::{client_sync::v24::Client, json::v24 as json};

#[cfg(all(feature = "24_1", not(feature = "24_2")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use bitcoind_json_rpc_client::{client_sync::v24::Client, json::v24 as json};

#[cfg(all(feature = "24_0_1", not(feature = "24_1")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use bitcoind_json_rpc_client::{client_sync::v24::Client, json::v24 as json};

#[cfg(all(feature = "23_2", not(feature = "24_0_1")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use bitcoind_json_rpc_client::{client_sync::v23::Client, json::v23 as json};

#[cfg(all(feature = "23_1", not(feature = "23_2")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use bitcoind_json_rpc_client::{client_sync::v23::Client, json::v23 as json};

#[cfg(all(feature = "23_0", not(feature = "23_1")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use bitcoind_json_rpc_client::{client_sync::v23::Client, json::v23 as json};

#[cfg(all(feature = "22_1", not(feature = "23_0")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use bitcoind_json_rpc_client::{client_sync::v22::Client, json::v22 as json};

#[cfg(all(feature = "22_0", not(feature = "22_1")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use bitcoind_json_rpc_client::{client_sync::v22::Client, json::v22 as json};

#[cfg(all(feature = "0_21_2", not(feature = "22_0")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use bitcoind_json_rpc_client::{client_sync::v21::Client, json::v21 as json};

#[cfg(all(feature = "0_20_2", not(feature = "0_21_2")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use bitcoind_json_rpc_client::{client_sync::v20::Client, json::v20 as json};

#[cfg(all(feature = "0_19_1", not(feature = "0_20_2")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use bitcoind_json_rpc_client::{client_sync::v19::Client, json::v19 as json};

#[cfg(all(feature = "0_18_1", not(feature = "0_19_1")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use bitcoind_json_rpc_client::{client_sync::v18::Client, json::v18 as json};

#[cfg(all(feature = "0_17_2", not(feature = "0_18_1")))]
#[allow(unused_imports)] // Not all users need the json types.
pub use bitcoind_json_rpc_client::{client_sync::v17::Client, json::v17 as json};
