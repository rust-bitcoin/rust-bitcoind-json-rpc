// SPDX-License-Identifier: CC0-1.0

//! Structs with standard types.
//!
//! These structs model the types returned by the JSON-RPC API and use stdlib types (or custom
//! types) and are specific to a specific to Bitcoin Core `v0.21.2`.
//!
//! **== Blockchain ==**
//! - [x] `getbestblockhash`
//! - [x] `getblock "blockhash" ( verbosity )`
//! - [x] `getblockchaininfo`
//! - [ ] `getblockcount`
//! - [ ] `getblockfilter "blockhash" ( "filtertype" )`
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
//! - [ ] `getrawmempool ( verbose mempool_sequence )`
//! - [ ] `gettxout "txid" n ( include_mempool )`
//! - [ ] `gettxoutproof ["txid",...] ( "blockhash" )`
//! - [ ] `gettxoutsetinfo ( "hash_type" )`
//! - [ ] `preciousblock "blockhash"`
//! - [ ] `pruneblockchain height`
//! - [ ] `savemempool`
//! - [ ] `scantxoutset "action" ( [scanobjects,...] )`
//! - [ ] `verifychain ( checklevel nblocks )`
//! - [ ] `verifytxoutproof "proof"`
//!
//! **== Control ==**
//! - [ ] `getmemoryinfo ( "mode" )`
//! - [ ] `getrpcinfo`
//! - [ ] `help ( "command" )`
//! - [ ] `logging ( ["include_category",...] ["exclude_category",...] )`
//! - [x] `stop`
//! - [ ] `uptime`
//!
//! **== Generating ==**
//! - [x] `generateblock "output" ["rawtx/txid",...]`
//! - [ ] `generatetoaddress nblocks "address" ( maxtries )`
//! - [ ] `generatetodescriptor num_blocks "descriptor" ( maxtries )`
//!
//! **== Mining ==**
//! - [ ] `getblocktemplate ( "template_request" )`
//! - [ ] `getmininginfo`
//! - [ ] `getnetworkhashps ( nblocks height )`
//! - [ ] `prioritisetransaction "txid" ( dummy ) fee_delta`
//! - [ ] `submitblock "hexdata" ( "dummy" )`
//! - [ ] `submitheader "hexdata"`
//!
//! **== Network ==**
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
//! **== Rawtransactions ==**
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
//! - [ ] `sendrawtransaction "hexstring" ( maxfeerate )`
//! - [ ] `signrawtransactionwithkey "hexstring" ["privatekey",...] ( [{"txid":"hex","vout":n,"scriptPubKey":"hex","redeemScript":"hex","witnessScript":"hex","amount":amount},...] "sighashtype" )`
//! - [ ] `testmempoolaccept ["rawtx",...] ( maxfeerate )`
//! - [ ] `utxoupdatepsbt "psbt" ( ["",{"desc":"str","range":n or [n,n]},...] )`
//!
//! **== Util ==**
//! - [ ] `createmultisig nrequired ["key",...] ( "address_type" )`
//! - [ ] `deriveaddresses "descriptor" ( range )`
//! - [ ] `estimatesmartfee conf_target ( "estimate_mode" )`
//! - [ ] `getdescriptorinfo "descriptor"`
//! - [ ] `getindexinfo ( "index_name" )`
//! - [ ] `signmessagewithprivkey "privkey" "message"`
//! - [ ] `validateaddress "address"`
//! - [ ] `verifymessage "address" "signature" "message"`
//!
//! **== Wallet ==**
//! - [ ] `abandontransaction "txid"`
//! - [ ] `abortrescan`
//! - [ ] `addmultisigaddress nrequired ["key",...] ( "label" "address_type" )`
//! - [ ] `backupwallet "destination"`
//! - [ ] `bumpfee "txid" ( options )`
//! - [x] `createwallet "wallet_name" ( disable_private_keys blank "passphrase" avoid_reuse descriptors load_on_startup )`
//! - [ ] `dumpprivkey "address"`
//! - [ ] `dumpwallet "filename"`
//! - [ ] `encryptwallet "passphrase"`
//! - [ ] `getaddressesbylabel "label"`
//! - [ ] `getaddressinfo "address"`
//! - [x] `getbalance ( "dummy" minconf include_watchonly avoid_reuse )`
//! - [x] `getbalances`
//! - [x] `getnewaddress ( "label" "address_type" )`
//! - [ ] `getrawchangeaddress ( "address_type" )`
//! - [ ] `getreceivedbyaddress "address" ( minconf )`
//! - [ ] `getreceivedbylabel "label" ( minconf )`
//! - [x] `gettransaction "txid" ( include_watchonly verbose )`
//! - [ ] `getunconfirmedbalance`
//! - [ ] `getwalletinfo`
//! - [ ] `importaddress "address" ( "label" rescan p2sh )`
//! - [ ] `importdescriptors "requests"`
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
//! - [x] `loadwallet "filename" ( load_on_startup )`
//! - [ ] `lockunspent unlock ( [{"txid":"hex","vout":n},...] )`
//! - [ ] `psbtbumpfee "txid" ( options )`
//! - [ ] `removeprunedfunds "txid"`
//! - [ ] `rescanblockchain ( start_height stop_height )`
//! - [ ] `send [{"address":amount},{"data":"hex"},...] ( conf_target "estimate_mode" fee_rate options )`
//! - [ ] `sendmany "" {"address":amount} ( minconf "comment" ["address",...] replaceable conf_target "estimate_mode" fee_rate verbose )`
//! - [x] `sendtoaddress "address" amount ( "comment" "comment_to" subtractfeefromamount replaceable conf_target "estimate_mode" avoid_reuse fee_rate verbose )`
//! - [ ] `sethdseed ( newkeypool "seed" )`
//! - [ ] `setlabel "address" "label"`
//! - [ ] `settxfee amount`
//! - [ ] `setwalletflag "flag" ( value )`
//! - [ ] `signmessage "address" "message"`
//! - [ ] `signrawtransactionwithwallet "hexstring" ( [{"txid":"hex","vout":n,"scriptPubKey":"hex","redeemScript":"hex","witnessScript":"hex","amount":amount},...] "sighashtype" )`
//! - [ ] `unloadwallet ( "wallet_name" load_on_startup )`
//! - [ ] `upgradewallet ( version )`
//! - [ ] `walletcreatefundedpsbt ( [{"txid":"hex","vout":n,"sequence":n},...] ) [{"address":amount},{"data":"hex"},...] ( locktime options bip32derivs )`
//! - [ ] `walletlock`
//! - [ ] `walletpassphrase "passphrase" timeout`
//! - [ ] `walletpassphrasechange "oldpassphrase" "newpassphrase"`
//! - [ ] `walletprocesspsbt "psbt" ( sign "sighashtype" bip32derivs )`
//!
//! **== Zmq ==**
//! - [ ] `getzmqnotifications`

#[doc(inline)]
pub use crate::{
    v17::{
        CreateWallet, GenerateToAddress, GetBalance, GetBestBlockHash, GetBlockVerbosityOne,
        GetBlockVerbosityZero, GetNetworkInfo, GetNetworkInfoAddress, GetNetworkInfoNetwork,
        GetNewAddress, GetTransaction, GetTransactionDetail, GetTransactionDetailCategory,
        GetTxOut, LoadWallet, SendRawTransaction, SendToAddress,
    },
    v19::{
        Bip9SoftforkInfo, Bip9SoftforkStatistics, Bip9SoftforkStatus, GetBalances, GetBalancesMine,
        GetBalancesWatchOnly, GetBlockchainInfo, Softfork, SoftforkType,
    },
};
