// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the wallet section of the API docs.
//!
//! The JSON-RPC API for Bitcoin Core v26.1:
//!
//!  == Network ==
//!
//! - [ ] addnode "node" "command" ( v2transport )
//! - [ ] clearbanned
//! - [ ] disconnectnode ( "address" nodeid )
//! - [ ] getaddednodeinfo ( "node" )
//! - [ ] getaddrmaninfo
//! - [ ] getconnectioncount
//! - [ ] getnettotals
//! - [x] getnetworkinfo
//! - [ ] getnodeaddresses ( count "network" )
//! - [ ] getpeerinfo
//! - [ ] listbanned
//! - [ ] ping
//! - [ ] setban "subnet" "command" ( bantime absolute )
//! - [ ] setnetworkactive state

pub use crate::json::v22::network::{GetNetworkInfo, GetNetworkInfoAddress, GetNetworkInfoNetwork};
