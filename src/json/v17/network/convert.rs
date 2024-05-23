// SPDX-License-Identifier: CC0-1.0

//! Convert stdlib (version specific) types to concrete types.
//!
//! This module does the conversion for `v0.17.1` types to the general concrete types.

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::{Amount, FeeRate};
use internals::write_err;

use crate::json::v17;
use crate::model;

// TODO: Upstream to `rust-bitcoin`.
/// Constructs a `bitcoin::FeeRate` from bitcoin per 1000 bytes.
fn fee_rate_from_btc_per_kb(btc_kb: f64) -> Result<FeeRate, ParseAmountError> {
    let amount = Amount::from_btc(btc_kb)?;
    let sat_kb = amount.to_sat();
    // There were no virtual bytes in v0.17.1
    Ok(FeeRate::from_sat_per_kwu(sat_kb))
}

impl TryFrom<v17::GetNetworkInfo> for model::GetNetworkInfo {
    type Error = GetNetworkInfoError;

    fn try_from(json: v17::GetNetworkInfo) -> Result<Self, Self::Error> {
        use GetNetworkInfoError as E;

        let relay_fee = fee_rate_from_btc_per_kb(json.relay_fee).map_err(E::RelayFee)?;
        let incremental_fee =
            fee_rate_from_btc_per_kb(json.incremental_fee).map_err(E::IncrementalFee)?;

        Ok(Self {
            version: json.version,
            subversion: json.subversion,
            protocol_version: json.protocol_version,
            local_services: json.local_services,
            local_services_names: vec![], // TODO: Manually create names?
            local_relay: json.local_relay,
            time_offset: json.time_offset,
            connections: json.connections,
            connections_in: 0,  // FIXME: Can we do better than this?
            connections_out: 0, // FIXME: Can we do better than this?
            network_active: json.network_active,
            networks: json.networks.into_iter().map(From::from).collect(),
            relay_fee,
            incremental_fee,
            local_addresses: json.local_addresses.into_iter().map(From::from).collect(),
            warnings: json.warnings,
        })
    }
}

impl From<v17::GetNetworkInfoNetwork> for model::GetNetworkInfoNetwork {
    fn from(json: v17::GetNetworkInfoNetwork) -> Self {
        Self {
            name: json.name,
            limited: json.limited,
            reachable: json.reachable,
            proxy: json.proxy,
            proxy_randomize_credentials: json.proxy_randomize_credentials,
        }
    }
}

impl From<v17::GetNetworkInfoAddress> for model::GetNetworkInfoAddress {
    fn from(json: v17::GetNetworkInfoAddress) -> Self {
        Self { address: json.address, port: json.port, score: json.score }
    }
}

/// Error when converting to a `v22::GetBlockchainInfo` type to a `concrete` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetNetworkInfoError {
    /// Conversion of the `relay_fee` field failed.
    RelayFee(ParseAmountError),
    /// Conversion of the `incremental_fee` field failed.
    IncrementalFee(ParseAmountError),
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

#[cfg(feature = "std")]
impl std::error::Error for GetNetworkInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetNetworkInfoError::*;

        match *self {
            RelayFee(ref e) => Some(e),
            IncrementalFee(ref e) => Some(e),
        }
    }
}
