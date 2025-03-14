//!
//! Wallet framework network parameters that control maturity
//! durations and other transaction related properties.
//!

use crate::imports::*;
use entropyx_consensus_core::mass::Kip9Version;
// ENX-CHANGE-BURN:
use entropyx_consensus_core::config::params::{
    MAINNET_PARAMS, TESTNET_PARAMS, TESTNET11_PARAMS, DEVNET_PARAMS, SIMNET_PARAMS
};

#[derive(Debug)]
pub struct NetworkParams {
    pub coinbase_transaction_maturity_period_daa: AtomicU64,
    pub coinbase_transaction_stasis_period_daa: u64,
    pub user_transaction_maturity_period_daa: AtomicU64,
    pub kip9_version: Kip9Version,
    pub additional_compound_transaction_mass: u64,
    // ENX-CHANGE-BURN:
    pub initial_burn_fee: u64,
    pub current_burn_fee: AtomicU64,
    pub burn_fee_activation_daa_score: u64,
}

impl NetworkParams {
    #[inline]
    pub fn coinbase_transaction_maturity_period_daa(&self) -> u64 {
        self.coinbase_transaction_maturity_period_daa.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn coinbase_transaction_stasis_period_daa(&self) -> u64 {
        self.coinbase_transaction_stasis_period_daa
    }

    #[inline]
    pub fn user_transaction_maturity_period_daa(&self) -> u64 {
        self.user_transaction_maturity_period_daa.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn kip9_version(&self) -> Kip9Version {
        self.kip9_version
    }

    #[inline]
    pub fn additional_compound_transaction_mass(&self) -> u64 {
        self.additional_compound_transaction_mass
    }

    pub fn set_coinbase_transaction_maturity_period_daa(&self, value: u64) {
        self.coinbase_transaction_maturity_period_daa.store(value, Ordering::Relaxed);
    }

    pub fn set_user_transaction_maturity_period_daa(&self, value: u64) {
        self.user_transaction_maturity_period_daa.store(value, Ordering::Relaxed);
    }

    /// BURN-CHANGE-BURN: 
    /// Get the current burn fee
    #[inline]
    pub fn get_current_burn_fee(&self) -> u64 {
        self.current_burn_fee.load(Ordering::Relaxed)
    }

    /// BURN-CHANGE-BURN: 
    /// Update the current burn fee based on the number of halvings
    pub fn update_current_burn_fee(&self, current_daa_score: u64) {
        let years_since_activation = current_daa_score / self.burn_fee_activation_daa_score;
        
        let new_burn_fee = self.initial_burn_fee >> years_since_activation;
        self.current_burn_fee.store(new_burn_fee, Ordering::Relaxed);
    }
}

static MAINNET_NETWORK_PARAMS: LazyLock<NetworkParams> = LazyLock::new(|| NetworkParams {
    coinbase_transaction_maturity_period_daa: AtomicU64::new(100),
    coinbase_transaction_stasis_period_daa: 50,
    user_transaction_maturity_period_daa: AtomicU64::new(10),
    kip9_version: Kip9Version::Beta,
    additional_compound_transaction_mass: 100,
    // ENX-CHANGE-BURN:
    initial_burn_fee: MAINNET_PARAMS.burn_fee_calculator.initial_burn_fee,
    current_burn_fee: AtomicU64::new(0),
    burn_fee_activation_daa_score: MAINNET_PARAMS.burn_fee_calculator.deflationary_phase_daa_score,
});

static TESTNET10_NETWORK_PARAMS: LazyLock<NetworkParams> = LazyLock::new(|| NetworkParams {
    coinbase_transaction_maturity_period_daa: AtomicU64::new(100),
    coinbase_transaction_stasis_period_daa: 50,
    user_transaction_maturity_period_daa: AtomicU64::new(10),
    kip9_version: Kip9Version::Beta,
    additional_compound_transaction_mass: 100,
    // ENX-CHANGE-BURN:
    initial_burn_fee: TESTNET_PARAMS.burn_fee_calculator.initial_burn_fee,
    current_burn_fee: AtomicU64::new(0),
    burn_fee_activation_daa_score: TESTNET_PARAMS.burn_fee_calculator.deflationary_phase_daa_score,
});

static TESTNET11_NETWORK_PARAMS: LazyLock<NetworkParams> = LazyLock::new(|| NetworkParams {
    coinbase_transaction_maturity_period_daa: AtomicU64::new(1_000),
    coinbase_transaction_stasis_period_daa: 500,
    user_transaction_maturity_period_daa: AtomicU64::new(100),
    kip9_version: Kip9Version::Beta,
    additional_compound_transaction_mass: 100,
    // ENX-CHANGE-BURN:
    initial_burn_fee: TESTNET11_PARAMS.burn_fee_calculator.initial_burn_fee,
    current_burn_fee: AtomicU64::new(0),
    burn_fee_activation_daa_score: TESTNET11_PARAMS.burn_fee_calculator.deflationary_phase_daa_score,
});

static SIMNET_NETWORK_PARAMS: LazyLock<NetworkParams> = LazyLock::new(|| NetworkParams {
    coinbase_transaction_maturity_period_daa: AtomicU64::new(100),
    coinbase_transaction_stasis_period_daa: 50,
    user_transaction_maturity_period_daa: AtomicU64::new(10),
    kip9_version: Kip9Version::Beta,
    additional_compound_transaction_mass: 0,
    // ENX-CHANGE-BURN:
    initial_burn_fee: SIMNET_PARAMS.burn_fee_calculator.initial_burn_fee,
    current_burn_fee: AtomicU64::new(0),
    burn_fee_activation_daa_score: SIMNET_PARAMS.burn_fee_calculator.deflationary_phase_daa_score,
});

static DEVNET_NETWORK_PARAMS: LazyLock<NetworkParams> = LazyLock::new(|| NetworkParams {
    coinbase_transaction_maturity_period_daa: AtomicU64::new(100),
    coinbase_transaction_stasis_period_daa: 50,
    user_transaction_maturity_period_daa: AtomicU64::new(10),
    kip9_version: Kip9Version::Beta,
    additional_compound_transaction_mass: 0,
    // ENX-CHANGE-BURN:
    initial_burn_fee: DEVNET_PARAMS.burn_fee_calculator.initial_burn_fee,
    current_burn_fee: AtomicU64::new(0),
    burn_fee_activation_daa_score: DEVNET_PARAMS.burn_fee_calculator.deflationary_phase_daa_score,
});

impl NetworkParams {
    pub fn from(value: NetworkId) -> &'static NetworkParams {
        match value.network_type {
            NetworkType::Mainnet => &MAINNET_NETWORK_PARAMS,
            NetworkType::Testnet => match value.suffix {
                Some(10) => &TESTNET10_NETWORK_PARAMS,
                Some(11) => &TESTNET11_NETWORK_PARAMS,
                Some(x) => panic!("Testnet suffix {} is not supported", x),
                None => panic!("Testnet suffix not provided"),
            },
            NetworkType::Devnet => &DEVNET_NETWORK_PARAMS,
            NetworkType::Simnet => &SIMNET_NETWORK_PARAMS,
        }
    }
}

/// Set the coinbase transaction maturity period DAA score for a given network.
/// This controls the DAA period after which the user transactions are considered mature
/// and the wallet subsystem emits the transaction maturity event.
pub fn set_coinbase_transaction_maturity_period_daa(network_id: &NetworkId, value: u64) {
    let network_params = NetworkParams::from(*network_id);
    if value <= network_params.coinbase_transaction_stasis_period_daa() {
        panic!(
            "Coinbase transaction maturity period must be greater than the stasis period of {} DAA",
            network_params.coinbase_transaction_stasis_period_daa()
        );
    }
    network_params.set_coinbase_transaction_maturity_period_daa(value);
}

/// Set the user transaction maturity period DAA score for a given network.
/// This controls the DAA period after which the user transactions are considered mature
/// and the wallet subsystem emits the transaction maturity event.
pub fn set_user_transaction_maturity_period_daa(network_id: &NetworkId, value: u64) {
    let network_params = NetworkParams::from(*network_id);
    if value == 0 {
        panic!("User transaction maturity period must be greater than 0");
    }
    network_params.set_user_transaction_maturity_period_daa(value);
}
