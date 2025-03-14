use entropyx_hashes::Hash;
use serde::{Deserialize, Serialize};
use entropyx_utils::mem_size::MemSizeEstimator;

/// ENX-CHANGE-BURN: Burn record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnRecord {
    pub block_hash: Hash,
    pub block_daa_score: u64,
    pub burned_value: u64,
    pub total_burned: u64,
    pub timestamp: u64,
}

impl MemSizeEstimator for BurnRecord {
    fn estimate_mem_bytes(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

/// ENX-CHANGE-BURN: Burn fee calculator
#[derive(Debug)]
#[derive(Clone)]
pub struct BurnFeeCalculator{
    pub initial_burn_fee: u64,
    pub deflationary_phase_daa_score: u64,
}

impl BurnFeeCalculator {
    /// ENX-CHANGE-BURN: 
    /// Compute the current burn fee based on the number of halvings
    pub fn get_current_burn_fee(&self, current_daa_score: u64) -> u64 {
        let years_since_activation = current_daa_score / self.deflationary_phase_daa_score;

        if years_since_activation >= 64 {
            return 0;
        }
        
        return self.initial_burn_fee >> years_since_activation;
    }
}