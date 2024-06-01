// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the wallet section of the API docs.
//!
//! The JSON-RPC API for Bitcoin Core v22.1:
//!
//!  == Network ==
//!
//! - [ ] addnode "node" "command"
//! - [ ] clearbanned
//! - [ ] disconnectnode ( "address" nodeid )
//! - [ ] getaddednodeinfo ( "node" )
//! - [ ] getconnectioncount
//! - [ ] getnettotals
//! - [x] getnetworkinfo
//! - [ ] getnodeaddresses ( count "network" )
//! - [ ] getpeerinfo
//! - [ ] listbanned
//! - [ ] ping
//! - [ ] setban "subnet" "command" ( bantime absolute )
//! - [ ] setnetworkactive state

mod convert;

use serde::{Deserialize, Serialize};

/// Result of the JSON-RPC method `getnetworkinfo`
///
/// > getnetworkinfo
///
/// > Returns an object containing various state info regarding P2P networking.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetNetworkInfo {
    /// The server version.
    pub version: usize,
    /// The server subversion string.
    pub subversion: String,
    /// The protocol version.
    #[serde(rename = "protocolversion")]
    pub protocol_version: usize,
    /// The services we offer to the network (hex string).
    #[serde(rename = "localservices")]
    pub local_services: String,
    /// The services we offer to the network, in human-readable form.
    #[serde(rename = "localservicesnames")]
    pub local_services_names: Vec<String>,
    /// `true` if transaction relay is requested from peers.
    #[serde(rename = "localrelay")]
    pub local_relay: bool,
    /// The time offset.
    #[serde(rename = "timeoffset")]
    pub time_offset: isize,
    /// The total number of connections.
    pub connections: usize,
    /// The number of inbound connections.
    pub connections_in: usize,
    /// The number of outbound connections.
    pub connections_out: usize,
    #[serde(rename = "networkactive")]
    /// Whether p2p networking is enabled.
    pub network_active: bool,
    /// Information per network.
    pub networks: Vec<GetNetworkInfoNetwork>,
    /// Minimum relay fee rate for transactions in BTC/kvB.
    #[serde(rename = "relayfee")]
    pub relay_fee: f64,
    /// Minimum fee rate increment for mempool limiting or replacement in BTC/kvB.
    #[serde(rename = "incrementalfee")]
    pub incremental_fee: f64,
    /// List of local addresses.
    #[serde(rename = "localaddresses")]
    pub local_addresses: Vec<GetNetworkInfoAddress>,
    /// Any network and blockchain warnings.
    pub warnings: String,
}

/// Part of the result of the JSON-RPC method `getnetworkinfo` (information per network).
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetNetworkInfoNetwork {
    /// Network (ipv4, ipv6, onion, i2p, cjdns).
    pub name: String,
    /// Is the network limited using -onlynet?.
    pub limited: bool,
    /// Is the network reachable?
    pub reachable: bool,
    /// ("host:port"): The proxy that is used for this network, or empty if none.
    pub proxy: String,
    /// Whether randomized credentials are used.
    pub proxy_randomize_credentials: bool,
}

/// Part of the result of the JSON-RPC method `getnetworkinfo` (local address info).
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetNetworkInfoAddress {
    /// Network address
    pub address: String,
    /// Network port
    pub port: u16,
    /// Relative score
    pub score: u32,
}
