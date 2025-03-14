pub mod acceptance_data;
pub mod block_transactions;
pub mod block_window_cache;
pub mod children;
pub mod daa;
pub mod depth;
pub mod ghostdag;
pub mod headers;
pub mod headers_selected_tip;
pub mod past_pruning_points;
pub mod pruning;
pub mod pruning_utxoset;
pub mod reachability;
pub mod relations;
pub mod selected_chain;
pub mod statuses;
pub mod tips;
pub mod utxo_diffs;
pub mod utxo_multisets;
pub mod utxo_set;
pub mod virtual_state;

pub use entropyx_database;
pub use entropyx_database::prelude::DB;
pub mod burned_fees; // ENX-CHANGE-BURN:
use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) struct U64Key([u8; size_of::<u64>()]);

impl From<u64> for U64Key {
    fn from(value: u64) -> Self {
        Self(value.to_le_bytes()) // TODO: Consider using big-endian for future ordering.
    }
}

impl AsRef<[u8]> for U64Key {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Display for U64Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u64::from_le_bytes(self.0))
    }
}

/// ENX-CHANGE-BURN: 
/// The u64 big key of the burned fees store
/// Ordering: big-endian
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) struct U64BigKey([u8; size_of::<u64>()]);

impl From<u64> for U64BigKey {
    fn from(value: u64) -> Self {
        Self(value.to_be_bytes())
    }
}

impl AsRef<[u8]> for U64BigKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Display for U64BigKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u64::from_be_bytes(self.0))
    }
}
