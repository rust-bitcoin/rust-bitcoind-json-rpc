// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core v0.17.1 - network.
//!
//! Types for methods found under the `== Network ==` section of the API docs.

use core::fmt;

use bitcoin::{amount, Amount, FeeRate};
use internals::write_err;
use serde::{Deserialize, Serialize};

use crate::model;

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
    /// `true` if transaction relay is requested from peers.
    #[serde(rename = "localrelay")]
    pub local_relay: bool,
    /// The time offset.
    #[serde(rename = "timeoffset")]
    pub time_offset: isize,
    /// The total number of connections.
    pub connections: usize,
    #[serde(rename = "networkactive")]
    /// Whether p2p networking is enabled.
    pub network_active: bool,
    /// Information per network.
    pub networks: Vec<GetNetworkInfoNetwork>,
    /// Minimum relay fee rate for transactions in BTC/kB.
    #[serde(rename = "relayfee")]
    pub relay_fee: f64,
    /// Minimum fee rate increment for mempool limiting or replacement in BTC/kB.
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

impl GetNetworkInfo {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetNetworkInfo, GetNetworkInfoError> {
        use GetNetworkInfoError as E;

        let relay_fee = fee_rate_from_btc_per_kb(self.relay_fee).map_err(E::RelayFee)?;
        let incremental_fee =
            fee_rate_from_btc_per_kb(self.incremental_fee).map_err(E::IncrementalFee)?;

        Ok(model::GetNetworkInfo {
            version: self.version,
            subversion: self.subversion,
            protocol_version: self.protocol_version,
            local_services: self.local_services,
            local_services_names: vec![], // TODO: Manually create names?
            local_relay: self.local_relay,
            time_offset: self.time_offset,
            connections: self.connections,
            network_active: self.network_active,
            networks: self.networks.into_iter().map(|j| j.into_model()).collect(),
            relay_fee,
            incremental_fee,
            local_addresses: self.local_addresses.into_iter().map(|j| j.into_model()).collect(),
            warnings: self.warnings,
        })
    }
}

// TODO: Upstream to `rust-bitcoin`.
/// Constructs a `bitcoin::FeeRate` from bitcoin per 1000 bytes.
fn fee_rate_from_btc_per_kb(btc_kb: f64) -> Result<FeeRate, amount::ParseAmountError> {
    let amount = Amount::from_btc(btc_kb)?;
    let sat_kb = amount.to_sat();
    // There were no virtual bytes in v0.17.1
    Ok(FeeRate::from_sat_per_kwu(sat_kb))
}

impl GetNetworkInfoNetwork {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::GetNetworkInfoNetwork {
        model::GetNetworkInfoNetwork {
            name: self.name,
            limited: self.limited,
            reachable: self.reachable,
            proxy: self.proxy,
            proxy_randomize_credentials: self.proxy_randomize_credentials,
        }
    }
}

impl GetNetworkInfoAddress {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::GetNetworkInfoAddress {
        model::GetNetworkInfoAddress { address: self.address, port: self.port, score: self.score }
    }
}

/// Error when converting to a `v22::GetBlockchainInfo` type to a `concrete` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetNetworkInfoError {
    /// Conversion of the `relay_fee` field failed.
    RelayFee(amount::ParseAmountError),
    /// Conversion of the `incremental_fee` field failed.
    IncrementalFee(amount::ParseAmountError),
}

impl fmt::Display for GetNetworkInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetNetworkInfoError::*;

        match *self {
            RelayFee(ref e) => write_err!(f, "conversion of the `relay_fee` field failed"; e),
            IncrementalFee(ref e) =>
                write_err!(f, "conversion of the `incremental_fee` field failed"; e),
        }
    }
}

impl std::error::Error for GetNetworkInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetNetworkInfoError::*;

        match *self {
            RelayFee(ref e) => Some(e),
            IncrementalFee(ref e) => Some(e),
        }
    }
}
