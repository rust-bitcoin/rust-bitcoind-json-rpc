// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core v25 - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

use serde::{Deserialize, Serialize};

use crate::model;

/// Result of the JSON-RPC method `createwallet`.
///
/// > createwallet "wallet_name" ( disable_private_keys blank "passphrase" avoid_reuse descriptors load_on_startup external_signer )
///
/// > Creates and loads a new wallet.
///
/// > Arguments:
/// > 1. wallet_name             (string, required) The name for the new wallet. If this is a path, the wallet will be created at the path location.
/// > 2. disable_private_keys    (boolean, optional, default=false) Disable the possibility of private keys (only watchonlys are possible in this mode).
/// > 3. blank                   (boolean, optional, default=false) Create a blank wallet. A blank wallet has no keys or HD seed. One can be set using sethdseed.
/// > 4. passphrase              (string, optional) Encrypt the wallet with this passphrase.
/// > 5. avoid_reuse             (boolean, optional, default=false) Keep track of coin reuse, and treat dirty and clean coins differently with privacy considerations in mind.
/// > 6. descriptors             (boolean, optional, default=true) Create a native descriptor wallet. The wallet will use descriptors internally to handle address creation. Setting to "false" will create a legacy wallet; however, the legacy wallet type is being deprecated and support for creating and opening legacy wallets will be removed in the future.
/// > 7. load_on_startup         (boolean, optional) Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
/// > 8. external_signer         (boolean, optional, default=false) Use an external signer such as a hardware wallet. Requires -signer to be configured. Wallet creation will fail if keys cannot be fetched. Requires disable_private_keys and descriptors set to true.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CreateWallet {
    /// The wallet name if created successfully.
    ///
    /// If the wallet was created using a full path, the wallet_name will be the full path.
    pub name: String,
    /// Warning messages, if any, related to creating and loading the wallet.
    pub warnings: Option<Vec<String>>,
}

impl CreateWallet {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::CreateWallet {
        model::CreateWallet { name: self.name, warnings: self.warnings.unwrap_or_default() }
    }

    /// Returns the created wallet name.
    pub fn name(self) -> String { self.into_model().name }
}

/// Result of the JSON-RPC method `loadwallet`.
///
/// > loadwallet "filename" ( load_on_startup )
///
/// > Loads a wallet from a wallet file or directory.
/// > Note that all wallet command-line options used when starting bitcoind will be
/// > applied to the new wallet.
///
/// > Arguments:
/// > 1. filename           (string, required) The wallet directory or .dat file.
/// > 2. load_on_startup    (boolean, optional) Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LoadWallet {
    /// The wallet name if loaded successfully.
    pub name: String,
    /// Warning messages, if any, related to loading the wallet.
    pub warnings: Option<Vec<String>>,
}

impl LoadWallet {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::LoadWallet {
        model::LoadWallet { name: self.name, warnings: self.warnings.unwrap_or_default() }
    }

    /// Returns the loaded wallet name.
    pub fn name(self) -> String { self.into_model().name }
}
