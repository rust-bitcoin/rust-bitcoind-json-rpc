// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the wallet section of the API docs.
//!
//! The JSON-RPC API for Bitcoin Core v26.1:
//!
//!  == Wallet ==
//!
//! - [ ] abandontransaction "txid"
//! - [ ] abortrescan
//! - [ ] addmultisigaddress nrequired ["key",...] ( "label" "address_type" )
//! - [ ] backupwallet "destination"
//! - [ ] bumpfee "txid" ( options )
//! - [x] createwallet "wallet_name" ( disable_private_keys blank "passphrase" avoid_reuse descriptors load_on_startup external_signer )
//! - [ ] dumpprivkey "address"
//! - [ ] dumpwallet "filename"
//! - [ ] encryptwallet "passphrase"
//! - [ ] getaddressesbylabel "label"
//! - [ ] getaddressinfo "address"
//! - [x] getbalance ( "dummy" minconf include_watchonly avoid_reuse )
//! - [x] getbalances
//! - [ ] getnewaddress ( "label" "address_type" )
//! - [ ] getrawchangeaddress ( "address_type" )
//! - [ ] getreceivedbyaddress "address" ( minconf include_immature_coinbase )
//! - [ ] getreceivedbylabel "label" ( minconf include_immature_coinbase )
//! - [ ] gettransaction "txid" ( include_watchonly verbose )
//! - [ ] getunconfirmedbalance
//! - [ ] getwalletinfo
//! - [ ] importaddress "address" ( "label" rescan p2sh )
//! - [ ] importdescriptors requests
//! - [ ] importmulti requests ( options )
//! - [ ] importprivkey "privkey" ( "label" rescan )
//! - [ ] importprunedfunds "rawtransaction" "txoutproof"
//! - [ ] importpubkey "pubkey" ( "label" rescan )
//! - [ ] importwallet "filename"
//! - [ ] keypoolrefill ( newsize )
//! - [ ] listaddressgroupings
//! - [ ] listdescriptors ( private )
//! - [ ] listlabels ( "purpose" )
//! - [ ] listlockunspent
//! - [ ] listreceivedbyaddress ( minconf include_empty include_watchonly "address_filter" include_immature_coinbase )
//! - [ ] listreceivedbylabel ( minconf include_empty include_watchonly include_immature_coinbase )
//! - [ ] listsinceblock ( "blockhash" target_confirmations include_watchonly include_removed include_change "label" )
//! - [ ] listtransactions ( "label" count skip include_watchonly )
//! - [ ] listunspent ( minconf maxconf ["address",...] include_unsafe query_options )
//! - [ ] listwalletdir
//! - [ ] listwallets
//! - [x] loadwallet "filename" ( load_on_startup )
//! - [ ] lockunspent unlock ( [{"txid":"hex","vout":n},...] persistent )
//! - [ ] migratewallet ( "wallet_name" "passphrase" )
//! - [ ] newkeypool
//! - [ ] psbtbumpfee "txid" ( options )
//! - [ ] removeprunedfunds "txid"
//! - [ ] rescanblockchain ( start_height stop_height )
//! - [ ] restorewallet "wallet_name" "backup_file" ( load_on_startup )
//! - [ ] send [{"address":amount,...},{"data":"hex"},...] ( conf_target "estimate_mode" fee_rate options )
//! - [ ] sendall ["address",{"address":amount,...},...] ( conf_target "estimate_mode" fee_rate options )
//! - [ ] sendmany ( "" ) {"address":amount,...} ( minconf "comment" ["address",...] replaceable conf_target "estimate_mode" fee_rate verbose )
//! - [ ] sendtoaddress "address" amount ( "comment" "comment_to" subtractfeefromamount replaceable conf_target "estimate_mode" avoid_reuse fee_rate verbose )
//! - [ ] sethdseed ( newkeypool "seed" )
//! - [ ] setlabel "address" "label"
//! - [ ] settxfee amount
//! - [ ] setwalletflag "flag" ( value )
//! - [ ] signmessage "address" "message"
//! - [ ] signrawtransactionwithwallet "hexstring" ( [{"txid":"hex","vout":n,"scriptPubKey":"hex","redeemScript":"hex","witnessScript":"hex","amount":amount},...] "sighashtype" )
//! - [ ] simulaterawtransaction ( ["rawtx",...] {"include_watchonly":bool,...} )
//! - [x] unloadwallet ( "wallet_name" load_on_startup )
//! - [ ] upgradewallet ( version )
//! - [ ] walletcreatefundedpsbt ( [{"txid":"hex","vout":n,"sequence":n,"weight":n},...] ) [{"address":amount,...},{"data":"hex"},...] ( locktime options bip32derivs )
//! - [ ] walletdisplayaddress "address"
//! - [ ] walletlock
//! - [ ] walletpassphrase "passphrase" timeout
//! - [ ] walletpassphrasechange "oldpassphrase" "newpassphrase"
//! - [ ] walletprocesspsbt "psbt" ( sign "sighashtype" bip32derivs finalize )

mod convert;

use serde::{Deserialize, Serialize};

#[rustfmt::skip]                // Keep public re-exports separate.
pub use crate::json::v17::wallet::{GetNewAddress, LoadWallet};
pub use crate::json::v22::wallet::{GetBalance, GetBalances, UnloadWallet};

/// Result of the JSON-RPC method `createwallet`.
///
/// > createwallet "wallet_name" ( disable_private_keys blank "passphrase" avoid_reuse descriptors load_on_startup external_signer )
/// >
/// > Creates and loads a new wallet.
/// >
/// > Arguments:
/// > 1. wallet_name             (string, required) The name for the new wallet. If this is a path, the wallet will be created at the path location.
/// > 2. disable_private_keys    (boolean, optional, default=false) Disable the possibility of private keys (only watchonlys are possible in this mode).
/// > 3. blank                   (boolean, optional, default=false) Create a blank wallet. A blank wallet has no keys or HD seed. One can be set using sethdseed.
/// > 4. passphrase              (string, optional) Encrypt the wallet with this passphrase.
/// > 5. avoid_reuse             (boolean, optional, default=false) Keep track of coin reuse, and treat dirty and clean coins differently with privacy considerations in mind.
/// > 6. descriptors             (boolean, optional, default=true) Create a native descriptor wallet. The wallet will use descriptors internally to handle address creation. Setting to "false" will create a legacy wallet; This is only possible with the -deprecatedrpc=create_bdb setting because, the legacy wallet type is being deprecated and support for creating and opening legacy wallets will be removed in the future.
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
