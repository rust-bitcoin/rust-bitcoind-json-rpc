// SPDX-License-Identifier: CC0-1.0

//! Structs with standard types.
//!
//! These structs model the types returned by the JSON-RPC API and use stdlib types (or custom
//! types) and are specific to a specific to Bitcoin Core `v0.17.1`.

/// JSON-RPC types by API section.
pub mod blockchain;
pub mod control;
pub mod generating;
pub mod mining;
pub mod network;
pub mod raw_transactions;
pub mod util;
pub mod wallet;
pub mod zmq;

// Re-export everything to make usage more ergonomic.
#[allow(unused_imports)] // TODO: Remove this.
pub use blockchain::*;
#[allow(unused_imports)] // TODO: Remove this.
pub use blockchain::*;
#[allow(unused_imports)] // TODO: Remove this.
pub use control::*;
#[allow(unused_imports)] // TODO: Remove this.
pub use generating::*;
#[allow(unused_imports)] // TODO: Remove this.
pub use mining::*;
#[allow(unused_imports)] // TODO: Remove this.
pub use network::*;
#[allow(unused_imports)] // TODO: Remove this.
pub use raw_transactions::*;
#[allow(unused_imports)] // TODO: Remove this.
pub use util::*;
#[allow(unused_imports)] // TODO: Remove this.
pub use wallet::*;
#[allow(unused_imports)] // TODO: Remove this.
pub use wallet::*;
#[allow(unused_imports)] // TODO: Remove this.
pub use zmq::*;
