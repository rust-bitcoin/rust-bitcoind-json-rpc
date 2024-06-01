// SPDX-License-Identifier: CC0-1.0

//! Structs with standard types.
//!
//! These structs model the types returned by the JSON-RPC API and use stdlib types (or custom
//! types) and are specific to a specific to Bitcoin Core `v0.18.1`.
//!
//! ** == Blockchain ==**
//! - [x] `getbestblockhash`
//! - [x] `getblock "blockhash" ( verbosity )`
//! - [x] `getblockchaininfo`
//! - [ ] `getblockcount`
//! - [ ] `getblockhash height`
//! - [ ] `getblockheader "blockhash" ( verbose )`
//! - [ ] `getblockstats hash_or_height ( stats )`
//! - [ ] `getchaintips`
//! - [ ] `getchaintxstats ( nblocks "blockhash" )`
//! - [ ] `getdifficulty`
//! - [ ] `getmempoolancestors "txid" ( verbose )`
//! - [ ] `getmempooldescendants "txid" ( verbose )`
//! - [ ] `getmempoolentry "txid"`
//! - [ ] `getmempoolinfo`
//! - [ ] `getrawmempool ( verbose )`
//! - [ ] `gettxout "txid" n ( include_mempool )`
//! - [ ] `gettxoutproof ["txid",...] ( "blockhash" )`
//! - [ ] `gettxoutsetinfo`
//! - [ ] `preciousblock "blockhash"`
//! - [ ] `pruneblockchain height`
//! - [ ] `savemempool`
//! - [ ] `scantxoutset "action" [scanobjects,...]`
//! - [ ] `verifychain ( checklevel nblocks )`
//! - [ ] `verifytxoutproof "proof"`
//!
//! ** == Control ==**
//! - [ ] `getmemoryinfo ( "mode" )`
//! - [ ] `getrpcinfo`
//! - [ ] `help ( "command" )`
//! - [ ] `logging ( ["include_category",...] ["exclude_category",...] )`
//! - [x] `stop`
//! - [ ] `uptime`
//!
//! ** == Generating ==**
//! - [ ] `generate nblocks ( maxtries )`
//! - [x] `generatetoaddress nblocks "address" ( maxtries )`
//!
//! ** == Mining ==**
//! - [ ] `getblocktemplate "template_request"`
//! - [ ] `getmininginfo`
//! - [ ] `getnetworkhashps ( nblocks height )`
//! - [ ] `prioritisetransaction "txid" ( dummy ) fee_delta`
//! - [ ] `submitblock "hexdata" ( "dummy" )`
//! - [ ] `submitheader "hexdata"`
//!
//! ** == Network ==**
//! - [ ] `addnode "node" "command"`
//! - [ ] `clearbanned`
//! - [ ] `disconnectnode ( "address" nodeid )`
//! - [ ] `getaddednodeinfo ( "node" )`
//! - [ ] `getconnectioncount`
//! - [ ] `getnettotals`
//! - [x] `getnetworkinfo`
//! - [ ] `getnodeaddresses ( count )`
//! - [ ] `getpeerinfo`
//! - [ ] `listbanned`
//! - [ ] `ping`
//! - [ ] `setban "subnet" "command" ( bantime absolute )`
//! - [ ] `setnetworkactive state`
//!
//! ** == Rawtransactions ==**
//! - [ ] `analyzepsbt "psbt"`
//! - [ ] `combinepsbt ["psbt",...]`
//! - [ ] `combinerawtransaction ["hexstring",...]`
//! - [ ] `converttopsbt "hexstring" ( permitsigdata iswitness )`
//! - [ ] `createpsbt [{"txid":"hex","vout":n,"sequence":n},...] [{"address":amount},{"data":"hex"},...] ( locktime replaceable )`
//! - [ ] `createrawtransaction [{"txid":"hex","vout":n,"sequence":n},...] [{"address":amount},{"data":"hex"},...] ( locktime replaceable )`
//! - [ ] `decodepsbt "psbt"`
//! - [ ] `decoderawtransaction "hexstring" ( iswitness )`
//! - [ ] `decodescript "hexstring"`
//! - [ ] `finalizepsbt "psbt" ( extract )`
//! - [ ] `fundrawtransaction "hexstring" ( options iswitness )`
//! - [ ] `getrawtransaction "txid" ( verbose "blockhash" )`
//! - [ ] `joinpsbts ["psbt",...]`
//! - [ ] `sendrawtransaction "hexstring" ( allowhighfees )`
//! - [ ] `signrawtransactionwithkey "hexstring" ["privatekey",...] ( [{"txid":"hex","vout":n,"scriptPubKey":"hex","redeemScript":"hex","witnessScript":"hex","amount":amount},...] "sighashtype" )`
//! - [ ] `testmempoolaccept ["rawtx",...] ( allowhighfees )`
//! - [ ] `utxoupdatepsbt "psbt"`
//!
//! ** == Util ==**
//! - [ ] `createmultisig nrequired ["key",...] ( "address_type" )`
//! - [ ] `deriveaddresses "descriptor" ( range )`
//! - [ ] `estimatesmartfee conf_target ( "estimate_mode" )`
//! - [ ] `getdescriptorinfo "descriptor"`
//! - [ ] `signmessagewithprivkey "privkey" "message"`
//! - [ ] `validateaddress "address"`
//! - [ ] `verifymessage "address" "signature" "message"`
//!
//! ** == Wallet ==**
//! - [ ] `abandontransaction "txid"`
//! - [ ] `abortrescan`
//! - [ ] `addmultisigaddress nrequired ["key",...] ( "label" "address_type" )`
//! - [ ] `backupwallet "destination"`
//! - [ ] `bumpfee "txid" ( options )`
//! - [x] `createwallet "wallet_name" ( disable_private_keys blank )`
//! - [ ] `dumpprivkey "address"`
//! - [ ] `dumpwallet "filename"`
//! - [ ] `encryptwallet "passphrase"`
//! - [ ] `getaddressesbylabel "label"`
//! - [ ] `getaddressinfo "address"`
//! - [x] `getbalance ( "dummy" minconf include_watchonly )`
//! - [x] `getnewaddress ( "label" "address_type" )`
//! - [ ] `getrawchangeaddress ( "address_type" )`
//! - [ ] `getreceivedbyaddress "address" ( minconf )`
//! - [ ] `getreceivedbylabel "label" ( minconf )`
//! - [ ] `gettransaction "txid" ( include_watchonly )`
//! - [ ] `getunconfirmedbalance`
//! - [ ] `getwalletinfo`
//! - [ ] `importaddress "address" ( "label" rescan p2sh )`
//! - [ ] `importmulti "requests" ( "options" )`
//! - [ ] `importprivkey "privkey" ( "label" rescan )`
//! - [ ] `importprunedfunds "rawtransaction" "txoutproof"`
//! - [ ] `importpubkey "pubkey" ( "label" rescan )`
//! - [ ] `importwallet "filename"`
//! - [ ] `keypoolrefill ( newsize )`
//! - [ ] `listaddressgroupings`
//! - [ ] `listlabels ( "purpose" )`
//! - [ ] `listlockunspent`
//! - [ ] `listreceivedbyaddress ( minconf include_empty include_watchonly "address_filter" )`
//! - [ ] `listreceivedbylabel ( minconf include_empty include_watchonly )`
//! - [ ] `listsinceblock ( "blockhash" target_confirmations include_watchonly include_removed )`
//! - [ ] `listtransactions ( "label" count skip include_watchonly )`
//! - [ ] `listunspent ( minconf maxconf ["address",...] include_unsafe query_options )`
//! - [ ] `listwalletdir`
//! - [ ] `listwallets`
//! - [x] `loadwallet "filename"`
//! - [ ] `lockunspent unlock ( [{"txid":"hex","vout":n},...] )`
//! - [ ] `removeprunedfunds "txid"`
//! - [ ] `rescanblockchain ( start_height stop_height )`
//! - [ ] `sendmany "" {"address":amount} ( minconf "comment" ["address",...] replaceable conf_target "estimate_mode" )`
//! - [x] `sendtoaddress "address" amount ( "comment" "comment_to" subtractfeefromamount replaceable conf_target "estimate_mode" )`
//! - [ ] `sethdseed ( newkeypool "seed" )`
//! - [ ] `setlabel "address" "label"`
//! - [ ] `settxfee amount`
//! - [ ] `signmessage "address" "message"`
//! - [ ] `signrawtransactionwithwallet "hexstring" ( [{"txid":"hex","vout":n,"scriptPubKey":"hex","redeemScript":"hex","witnessScript":"hex","amount":amount},...] "sighashtype" )`
//! - [ ] `unloadwallet ( "wallet_name" )`
//! - [ ] `walletcreatefundedpsbt [{"txid":"hex","vout":n,"sequence":n},...] [{"address":amount},{"data":"hex"},...] ( locktime options bip32derivs )`
//! - [ ] `walletlock`
//! - [ ] `walletpassphrase "passphrase" timeout`
//! - [ ] `walletpassphrasechange "oldpassphrase" "newpassphrase"`
//! - [ ] `walletprocesspsbt "psbt" ( sign "sighashtype" bip32derivs )`
//! - [ ] `
//! - [ ] `//! ** == Zmq ==**`
//! - [ ] `getzmqnotifications`

// == Blockchain ==
// getbestblockhash
#[doc(inline)]
pub use crate::v17::GetBestBlockHash;
// getblock "blockhash" ( verbosity )
#[doc(inline)]
pub use crate::v17::{GetBlockVerbosityOne, GetBlockVerbosityZero};
// getblockchaininfo
#[doc(inline)]
pub use crate::v17::GetBlockchainInfo;
// getblockcount
// getblockhash height
// getblockheader "blockhash" ( verbose )
// getblockstats hash_or_height ( stats )
// getchaintips
// getchaintxstats ( nblocks "blockhash" )
// getdifficulty
// getmempoolancestors "txid" ( verbose )
// getmempooldescendants "txid" ( verbose )
// getmempoolentry "txid"
// getmempoolinfo
// getrawmempool ( verbose )
// gettxout "txid" n ( include_mempool )
#[doc(inline)]
pub use crate::v17::GetTxOut;
// gettxoutproof ["txid",...] ( "blockhash" )
// gettxoutsetinfo
// preciousblock "blockhash"
// pruneblockchain height
// savemempool
// scantxoutset "action" [scanobjects,...]
// verifychain ( checklevel nblocks )
// verifytxoutproof "proof"

// == Control ==
// getmemoryinfo ( "mode" )
// getrpcinfo
// help ( "command" )
// logging ( ["include_category",...] ["exclude_category",...] )
// uptime

// == Generating ==
// generate nblocks ( maxtries )
// generatetoaddress nblocks "address" ( maxtries )
#[doc(inline)]
pub use crate::v17::GenerateToAddress;

// == Mining ==
// getblocktemplate "template_request"
// getmininginfo
// getnetworkhashps ( nblocks height )
// prioritisetransaction "txid" ( dummy ) fee_delta
// submitblock "hexdata" ( "dummy" )
// submitheader "hexdata"

// == Network ==
// addnode "node" "command"
// clearbanned
// disconnectnode ( "address" nodeid )
// getaddednodeinfo ( "node" )
// getconnectioncount
// getnettotals
// getnetworkinfo
#[doc(inline)]
pub use crate::v17::GetNetworkInfo;
// getnodeaddresses ( count )
// getpeerinfo
// listbanned
// ping
// setban "subnet" "command" ( bantime absolute )
// setnetworkactive state

// == Rawtransactions ==
// analyzepsbt "psbt"
// combinepsbt ["psbt",...]
// combinerawtransaction ["hexstring",...]
// converttopsbt "hexstring" ( permitsigdata iswitness )
// createpsbt [{"txid":"hex","vout":n,"sequence":n},...] [{"address":amount},{"data":"hex"},...] ( locktime replaceable )
// createrawtransaction [{"txid":"hex","vout":n,"sequence":n},...] [{"address":amount},{"data":"hex"},...] ( locktime replaceable )
// decodepsbt "psbt"
// decoderawtransaction "hexstring" ( iswitness )
// decodescript "hexstring"
// finalizepsbt "psbt" ( extract )
// fundrawtransaction "hexstring" ( options iswitness )
// getrawtransaction "txid" ( verbose "blockhash" )
// joinpsbts ["psbt",...]
// sendrawtransaction "hexstring" ( allowhighfees )
// signrawtransactionwithkey "hexstring" ["privatekey",...] ( [{"txid":"hex","vout":n,"scriptPubKey":"hex","redeemScript":"hex","witnessScript":"hex","amount":amount},...] "sighashtype" )
// testmempoolaccept ["rawtx",...] ( allowhighfees )
// utxoupdatepsbt "psbt"

// == Util ==
// createmultisig nrequired ["key",...] ( "address_type" )
// deriveaddresses "descriptor" ( range )
// estimatesmartfee conf_target ( "estimate_mode" )
// getdescriptorinfo "descriptor"
// signmessagewithprivkey "privkey" "message"
// validateaddress "address"
// verifymessage "address" "signature" "message"

// == Wallet ==
// abandontransaction "txid"
// abortrescan
// addmultisigaddress nrequired ["key",...] ( "label" "address_type" )
// backupwallet "destination"
// bumpfee "txid" ( options )
// createwallet "wallet_name" ( disable_private_keys blank )
#[doc(inline)]
pub use crate::v17::CreateWallet;
// dumpprivkey "address"
// dumpwallet "filename"
// encryptwallet "passphrase"
// getaddressesbylabel "label"
// getaddressinfo "address"
// getbalance ( "dummy" minconf include_watchonly )
#[doc(inline)]
pub use crate::v17::GetBalance;
// getnewaddress ( "label" "address_type" )
#[doc(inline)]
pub use crate::v17::GetNewAddress;
// getrawchangeaddress ( "address_type" )
// getreceivedbyaddress "address" ( minconf )
// getreceivedbylabel "label" ( minconf )
// gettransaction "txid" ( include_watchonly )
// getunconfirmedbalance
// getwalletinfo
// importaddress "address" ( "label" rescan p2sh )
// importmulti "requests" ( "options" )
// importprivkey "privkey" ( "label" rescan )
// importprunedfunds "rawtransaction" "txoutproof"
// importpubkey "pubkey" ( "label" rescan )
// importwallet "filename"
// keypoolrefill ( newsize )
// listaddressgroupings
// listlabels ( "purpose" )
// listlockunspent
// listreceivedbyaddress ( minconf include_empty include_watchonly "address_filter" )
// listreceivedbylabel ( minconf include_empty include_watchonly )
// listsinceblock ( "blockhash" target_confirmations include_watchonly include_removed )
// listtransactions ( "label" count skip include_watchonly )
// listunspent ( minconf maxconf ["address",...] include_unsafe query_options )
// listwalletdir
// listwallets
// loadwallet "filename"
#[doc(inline)]
pub use crate::v17::LoadWallet;
// lockunspent unlock ( [{"txid":"hex","vout":n},...] )
// removeprunedfunds "txid"
// rescanblockchain ( start_height stop_height )
// sendmany "" {"address":amount} ( minconf "comment" ["address",...] replaceable conf_target "estimate_mode" )
// sendtoaddress "address" amount ( "comment" "comment_to" subtractfeefromamount replaceable conf_target "estimate_mode" )
#[doc(inline)]
pub use crate::v17::SendToAddress;
// sethdseed ( newkeypool "seed" )
// setlabel "address" "label"
// settxfee amount
// signmessage "address" "message"
// signrawtransactionwithwallet "hexstring" ( [{"txid":"hex","vout":n,"scriptPubKey":"hex","redeemScript":"hex","witnessScript":"hex","amount":amount},...] "sighashtype" )
// unloadwallet ( "wallet_name" )
// walletcreatefundedpsbt [{"txid":"hex","vout":n,"sequence":n},...] [{"address":amount},{"data":"hex"},...] ( locktime options bip32derivs )
// walletlock
// walletpassphrase "passphrase" timeout
// walletpassphrasechange "oldpassphrase" "newpassphrase"
// walletprocesspsbt "psbt" ( sign "sighashtype" bip32derivs )

// == Zmq ==
// getzmqnotifications
