// SPDX-License-Identifier: CC0-1.0

//! Structs with standard types.
//!
//! These structs model the types returned by the JSON-RPC API and use stdlib types (or custom
//! types) and are specific to a specific to Bitcoin Core `v26`.
//!
//! **== Blockchain ==**
//! - [ ] `dumptxoutset "path"`
//! - [x] `getbestblockhash`
//! - [x] `getblock "blockhash" ( verbosity )`
//! - [x] `getblockchaininfo`
//! - [ ] `getblockcount`
//! - [ ] `getblockfilter "blockhash" ( "filtertype" )`
//! - [ ] `getblockfrompeer "blockhash" peer_id`
//! - [ ] `getblockhash height`
//! - [ ] `getblockheader "blockhash" ( verbose )`
//! - [ ] `getblockstats hash_or_height ( stats )`
//! - [ ] `getchainstates`
//! - [ ] `getchaintips`
//! - [ ] `getchaintxstats ( nblocks "blockhash" )`
//! - [ ] `getdeploymentinfo ( "blockhash" )`
//! - [ ] `getdifficulty`
//! - [ ] `getmempoolancestors "txid" ( verbose )`
//! - [ ] `getmempooldescendants "txid" ( verbose )`
//! - [ ] `getmempoolentry "txid"`
//! - [ ] `getmempoolinfo`
//! - [ ] `getrawmempool ( verbose mempool_sequence )`
//! - [ ] `gettxout "txid" n ( include_mempool )`
//! - [ ] `gettxoutproof ["txid",...] ( "blockhash" )`
//! - [ ] `gettxoutsetinfo ( "hash_type" hash_or_height use_index )`
//! - [ ] `gettxspendingprevout [{"txid":"hex","vout":n},...]`
//! - [ ] `importmempool "filepath" ( options )`
//! - [ ] `loadtxoutset "path"`
//! - [ ] `preciousblock "blockhash"`
//! - [ ] `pruneblockchain height`
//! - [ ] `savemempool`
//! - [ ] `scanblocks "action" ( [scanobjects,...] start_height stop_height "filtertype" options )`
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
//! **== Mining ==**
//! - [ ] `getblocktemplate {"mode":"str","capabilities":["str",...],"rules":["segwit","str",...],"longpollid":"str","data":"hex"}`
//! - [ ] `getmininginfo`
//! - [ ] `getnetworkhashps ( nblocks height )`
//! - [ ] `getprioritisedtransactions`
//! - [ ] `prioritisetransaction "txid" ( dummy ) fee_delta`
//! - [ ] `submitblock "hexdata" ( "dummy" )`
//! - [ ] `submitheader "hexdata"`
//! - [ ] `//!`
//! - [ ] `//! **== Network ==**`
//! - [ ] `addnode "node" "command" ( v2transport )`
//! - [ ] `clearbanned`
//! - [ ] `disconnectnode ( "address" nodeid )`
//! - [ ] `getaddednodeinfo ( "node" )`
//! - [ ] `getaddrmaninfo`
//! - [ ] `getconnectioncount`
//! - [ ] `getnettotals`
//! - [ ] `getnetworkinfo`
//! - [ ] `getnodeaddresses ( count "network" )`
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
//! - [ ] `createpsbt [{"txid":"hex","vout":n,"sequence":n},...] [{"address":amount,...},{"data":"hex"},...] ( locktime replaceable )`
//! - [ ] `createrawtransaction [{"txid":"hex","vout":n,"sequence":n},...] [{"address":amount,...},{"data":"hex"},...] ( locktime replaceable )`
//! - [ ] `decodepsbt "psbt"`
//! - [ ] `decoderawtransaction "hexstring" ( iswitness )`
//! - [ ] `decodescript "hexstring"`
//! - [ ] `descriptorprocesspsbt "psbt" ["",{"desc":"str","range":n or [n,n]},...] ( "sighashtype" bip32derivs finalize )`
//! - [ ] `finalizepsbt "psbt" ( extract )`
//! - [ ] `fundrawtransaction "hexstring" ( options iswitness )`
//! - [ ] `getrawtransaction "txid" ( verbosity "blockhash" )`
//! - [ ] `joinpsbts ["psbt",...]`
//! - [ ] `sendrawtransaction "hexstring" ( maxfeerate maxburnamount )`
//! - [ ] `signrawtransactionwithkey "hexstring" ["privatekey",...] ( [{"txid":"hex","vout":n,"scriptPubKey":"hex","redeemScript":"hex","witnessScript":"hex","amount":amount},...] "sighashtype" )`
//! - [x] `submitpackage ["rawtx",...] ( maxfeerate maxburnamount )`
//! - [ ] `testmempoolaccept ["rawtx",...] ( maxfeerate )`
//! - [ ] `utxoupdatepsbt "psbt" ( ["",{"desc":"str","range":n or [n,n]},...] )`
//!
//! **== Signer ==**
//! - [ ] `enumeratesigners`
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
//! - [x] `createwallet "wallet_name" ( disable_private_keys blank "passphrase" avoid_reuse descriptors load_on_startup external_signer )`
//! - [ ] `createwalletdescriptor "type" ( {"internal":bool,"hdkey":"str",...} )`
//! - [ ] `dumpprivkey "address"`
//! - [ ] `dumpwallet "filename"`
//! - [ ] `encryptwallet "passphrase"`
//! - [ ] `getaddressesbylabel "label"`
//! - [ ] `getaddressinfo "address"`
//! - [x] `getbalance ( "dummy" minconf include_watchonly avoid_reuse )`
//! - [x] `getbalances`
//! - [ ] `gethdkeys ( {"active_only":bool,"private":bool,...} )`
//! - [x] `getnewaddress ( "label" "address_type" )`
//! - [ ] `getrawchangeaddress ( "address_type" )`
//! - [ ] `getreceivedbyaddress "address" ( minconf include_immature_coinbase )`
//! - [ ] `getreceivedbylabel "label" ( minconf include_immature_coinbase )`
//! - [x] `gettransaction "txid" ( include_watchonly verbose )`
//! - [ ] `getunconfirmedbalance`
//! - [ ] `getwalletinfo`
//! - [ ] `importaddress "address" ( "label" rescan p2sh )`
//! - [ ] `importdescriptors requests`
//! - [ ] `importmulti requests ( options )`
//! - [ ] `importprivkey "privkey" ( "label" rescan )`
//! - [ ] `importprunedfunds "rawtransaction" "txoutproof"`
//! - [ ] `importpubkey "pubkey" ( "label" rescan )`
//! - [ ] `importwallet "filename"`
//! - [ ] `keypoolrefill ( newsize )`
//! - [ ] `listaddressgroupings`
//! - [ ] `listdescriptors ( private )`
//! - [ ] `listlabels ( "purpose" )`
//! - [ ] `listlockunspent`
//! - [ ] `listreceivedbyaddress ( minconf include_empty include_watchonly "address_filter" include_immature_coinbase )`
//! - [ ] `listreceivedbylabel ( minconf include_empty include_watchonly include_immature_coinbase )`
//! - [ ] `listsinceblock ( "blockhash" target_confirmations include_watchonly include_removed include_change "label" )`
//! - [ ] `listtransactions ( "label" count skip include_watchonly )`
//! - [ ] `listunspent ( minconf maxconf ["address",...] include_unsafe query_options )`
//! - [ ] `listwalletdir`
//! - [ ] `listwallets`
//! - [x] `loadwallet "filename" ( load_on_startup )`
//! - [ ] `lockunspent unlock ( [{"txid":"hex","vout":n},...] persistent )`
//! - [ ] `migratewallet ( "wallet_name" "passphrase" )`
//! - [ ] `newkeypool`
//! - [ ] `psbtbumpfee "txid" ( options )`
//! - [ ] `removeprunedfunds "txid"`
//! - [ ] `rescanblockchain ( start_height stop_height )`
//! - [ ] `restorewallet "wallet_name" "backup_file" ( load_on_startup )`
//! - [ ] `send [{"address":amount,...},{"data":"hex"},...] ( conf_target "estimate_mode" fee_rate options )`
//! - [ ] `sendall ["address",{"address":amount,...},...] ( conf_target "estimate_mode" fee_rate options )`
//! - [ ] `sendmany ( "" ) {"address":amount,...} ( minconf "comment" ["address",...] replaceable conf_target "estimate_mode" fee_rate verbose )`
//! - [x] `sendtoaddress "address" amount ( "comment" "comment_to" subtractfeefromamount replaceable conf_target "estimate_mode" avoid_reuse fee_rate verbose )`
//! - [ ] `sethdseed ( newkeypool "seed" )`
//! - [ ] `setlabel "address" "label"`
//! - [ ] `settxfee amount`
//! - [ ] `setwalletflag "flag" ( value )`
//! - [ ] `signmessage "address" "message"`
//! - [ ] `signrawtransactionwithwallet "hexstring" ( [{"txid":"hex","vout":n,"scriptPubKey":"hex","redeemScript":"hex","witnessScript":"hex","amount":amount},...] "sighashtype" )`
//! - [ ] `simulaterawtransaction ( ["rawtx",...] {"include_watchonly":bool,...} )`
//! - [ ] `unloadwallet ( "wallet_name" load_on_startup )`
//! - [ ] `upgradewallet ( version )`
//! - [ ] `walletcreatefundedpsbt ( [{"txid":"hex","vout":n,"sequence":n,"weight":n},...] ) [{"address":amount,...},{"data":"hex"},...] ( locktime options bip32derivs )`
//! - [ ] `walletdisplayaddress "address"`
//! - [ ] `walletlock`
//! - [ ] `walletpassphrase "passphrase" timeout`
//! - [ ] `walletpassphrasechange "oldpassphrase" "newpassphrase"`
//! - [ ] `walletprocesspsbt "psbt" ( sign "sighashtype" bip32derivs finalize )`
//!
//! **== Zmq ==**
//! - [ ] `getzmqnotifications`

mod blockchain;
mod network;
mod raw_transactions;

#[doc(inline)]
pub use self::blockchain::GetBlockchainInfo;
#[doc(inline)]
pub use self::network::GetNetworkInfo;
#[doc(inline)]
pub use self::raw_transactions::{SubmitPackage, SubmitPackageTxResult, SubmitPackageTxResultFees};
#[doc(inline)]
pub use crate::{
    v17::{
        GenerateToAddress, GetBalance, GetBestBlockHash, GetBlockVerbosityOne,
        GetBlockVerbosityZero, GetNetworkInfoAddress, GetNetworkInfoError, GetNetworkInfoNetwork,
        GetNewAddress, GetTransaction, GetTransactionDetail, GetTransactionDetailCategory,
        GetTxOut, SendRawTransaction,
    },
    v19::{
        Bip9SoftforkInfo, Bip9SoftforkStatistics, Bip9SoftforkStatus, GetBalances, GetBalancesMine,
        GetBalancesWatchOnly, GetBlockchainInfoError, Softfork, SoftforkType,
    },
    v22::{SendToAddress, UnloadWallet},
    v25::{CreateWallet, LoadWallet},
};
