pub mod errors;
pub mod transaction_validator_populated;
mod tx_validation_in_isolation;
pub mod tx_validation_not_utxo_related;
use std::{collections::HashSet, sync::Arc}; // ENX-CHANGE-BURN:

use crate::model::stores::ghostdag;

use entropyx_core::debug;
use entropyx_txscript::{
    caches::{Cache, TxScriptCacheCounters},
    SigCacheKey,
};

use entropyx_consensus_core::{mass::MassCalculator, tx::VerifiableTransaction}; // ENX-CHANGE-BURN:

#[derive(Clone)]
pub struct TransactionValidator {
    max_tx_inputs: usize,
    max_tx_outputs: usize,
    max_signature_script_len: usize,
    max_script_public_key_len: usize,
    ghostdag_k: ghostdag::KType,
    coinbase_payload_script_public_key_max_len: u8,
    coinbase_maturity: u64,
    sig_cache: Cache<SigCacheKey, bool>,

    pub(crate) mass_calculator: MassCalculator,

    /// Storage mass hardfork DAA score
    storage_mass_activation_daa_score: u64,
}

impl TransactionValidator {
    pub fn new(
        max_tx_inputs: usize,
        max_tx_outputs: usize,
        max_signature_script_len: usize,
        max_script_public_key_len: usize,
        ghostdag_k: ghostdag::KType,
        coinbase_payload_script_public_key_max_len: u8,
        coinbase_maturity: u64,
        counters: Arc<TxScriptCacheCounters>,
        mass_calculator: MassCalculator,
        storage_mass_activation_daa_score: u64,
    ) -> Self {
        Self {
            max_tx_inputs,
            max_tx_outputs,
            max_signature_script_len,
            max_script_public_key_len,
            ghostdag_k,
            coinbase_payload_script_public_key_max_len,
            coinbase_maturity,
            sig_cache: Cache::with_counters(10_000, counters),
            mass_calculator,
            storage_mass_activation_daa_score,
        }
    }

    pub fn new_for_tests(
        max_tx_inputs: usize,
        max_tx_outputs: usize,
        max_signature_script_len: usize,
        max_script_public_key_len: usize,
        ghostdag_k: ghostdag::KType,
        coinbase_payload_script_public_key_max_len: u8,
        coinbase_maturity: u64,
        counters: Arc<TxScriptCacheCounters>,
    ) -> Self {
        Self {
            max_tx_inputs,
            max_tx_outputs,
            max_signature_script_len,
            max_script_public_key_len,
            ghostdag_k,
            coinbase_payload_script_public_key_max_len,
            coinbase_maturity,
            sig_cache: Cache::with_counters(10_000, counters),
            mass_calculator: MassCalculator::new(0, 0, 0, 0),
            storage_mass_activation_daa_score: u64::MAX,
        }
    }

    // ENX-CHANGE-BURN:
    // Check if the transaction is a same address transaction
    pub fn is_same_address_transaction(&self, tx: &impl VerifiableTransaction) -> bool {
        debug!("[SAME_ADDR]Checking transaction with {} inputs and {} outputs", 
            tx.populated_inputs().count(), tx.outputs().len());
    
        // collect and print all input addresses
        let input_addresses: Vec<_> = tx
            .populated_inputs()
            .map(|(_, entry)| entry.script_public_key.script_as_hex())
            .collect();
        
        // debug!("[SAME_ADDR]Input addresses:");
        // for (idx, addr) in input_addresses.iter().enumerate() {
        //     debug!("[SAME_ADDR]  Input {}: {}", idx, addr);
        // }
    
        // // print all output addresses
        // debug!("[SAME_ADDR]Output addresses:");
        // for (idx, output) in tx.outputs().iter().enumerate() {
        //     debug!("[SAME_ADDR]  Output {}: {}", idx, output.script_public_key.script_as_hex());
        // }
    
        // // if the number of outputs is not 1, return false
        // if tx.outputs().len() != 1 {
        //     debug!("[SAME_ADDR]Multiple outputs found, not a same address transaction");
        //     return false;
        // }
    
        // get the first input address
        if let Some(first_input_addr) = input_addresses.first() {
            let output_addr = tx.outputs()[0].script_public_key.script_as_hex();
            
            // check if all inputs match the first input address
            let all_inputs_match = input_addresses.iter().all(|addr| addr == first_input_addr);
            
            // check if the output matches the first input address
            let output_matches = &output_addr == first_input_addr;
    
            debug!("[SAME_ADDR]Comparison results:");
            debug!("[SAME_ADDR]  First input address: {}", first_input_addr);
            debug!("[SAME_ADDR]  Output address: {}", output_addr);
            debug!("[SAME_ADDR]  All inputs match: {}", all_inputs_match);
            debug!("[SAME_ADDR]  Output matches: {}", output_matches);
            debug!("[SAME_ADDR]  Final result: {}", all_inputs_match && output_matches);
    
            return all_inputs_match && output_matches;
        }
    
        debug!("[SAME_ADDR]No input addresses found, returning false");
        false
    }
}
