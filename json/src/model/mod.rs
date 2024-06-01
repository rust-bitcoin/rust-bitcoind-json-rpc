// SPDX-License-Identifier: CC0-1.0

//! Models of the data returned by the JSON-RPC API of Bitcoin Core.
//!
//! The types here model the data returned by Bitcoin Core in a version non-specific way. In other
//! words one can use a particular Bitcoin Core versions API via the [`crate::json`] module then
//! convert the `json` types to one of the modelled types in this module using `TryFrom`.

// Separated by the section name in the `bitcoind` JSON-RPC docs.
pub mod blockchain;
pub mod control;
pub mod generating;
pub mod mining;
pub mod network;
pub mod raw_transactions;
pub mod util;
pub mod wallet;
pub mod zmq;

// TODO/QUESTIONS
//
// - Should all types here be non_exhaustive (otherwise evertime Core changes them we will need a new major release)?
// - Should the modules be public?
// - Should we provide an inherent convertion method because try_from's lack of type inference is annoying to use.

// Re-export everything to make usage more ergonomic.
#[doc(inline)] // FIXME: Check this looks good when rendered.
pub use blockchain::*;
#[allow(unused_imports)] // TODO: Remove this.
pub use control::*;
#[allow(unused_imports)] // TODO: Remove this.
pub use generating::*;
#[allow(unused_imports)] // TODO: Remove this.
pub use mining::*;
pub use network::*;
#[allow(unused_imports)] // TODO: Remove this.
pub use raw_transactions::*;
#[allow(unused_imports)] // TODO: Remove this.
pub use util::*;
#[allow(unused_imports)] // TODO: Remove this.
pub use wallet::*;
#[allow(unused_imports)] // TODO: Remove this.
pub use zmq::*;
