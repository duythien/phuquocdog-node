use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{RuntimeDebug};

/// Enumerated asset on chain
#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum AssetId {
    /// PQD the native currency of the chain
    PHUQUOCDOG,
    /// Generic enumerated assed
    /// Range 0 - 0x00000000FFFFFFFF (2^32)-1 is reserved for protected tokens
    /// the values under 1000 are used for ISO 4217 Numeric Curency codes
    Asset(u64),
}