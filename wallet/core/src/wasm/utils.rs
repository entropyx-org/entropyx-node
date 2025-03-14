use crate::result::Result;
use js_sys::BigInt;
use entropyx_consensus_core::network::{NetworkType, NetworkTypeT};
use wasm_bindgen::prelude::*;
use workflow_wasm::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "bigint | number | HexString")]
    #[derive(Clone, Debug)]
    pub type ISompiToEntropyX;
}

/// Convert a EntropyX string to Sompi represented by bigint.
/// This function provides correct precision handling and
/// can be used to parse user input.
/// @category Wallet SDK
#[wasm_bindgen(js_name = "entropyxToSompi")]
pub fn entropyx_to_sompi(entropyx: String) -> Option<BigInt> {
    crate::utils::try_entropyx_str_to_sompi(entropyx).ok().flatten().map(Into::into)
}

///
/// Convert Sompi to a string representation of the amount in EntropyX.
///
/// @category Wallet SDK
///
#[wasm_bindgen(js_name = "sompiToEntropyXString")]
pub fn sompi_to_entropyx_string(sompi: ISompiToEntropyX) -> Result<String> {
    let sompi = sompi.try_as_u64()?;
    Ok(crate::utils::sompi_to_entropyx_string(sompi))
}

///
/// Format a Sompi amount to a string representation of the amount in EntropyX with a suffix
/// based on the network type (e.g. `ENX` for mainnet, `TENX` for testnet,
/// `SENX` for simnet, `DENX` for devnet).
///
/// @category Wallet SDK
///
#[wasm_bindgen(js_name = "sompiToEntropyXStringWithSuffix")]
pub fn sompi_to_entropyx_string_with_suffix(sompi: ISompiToEntropyX, network: &NetworkTypeT) -> Result<String> {
    let sompi = sompi.try_as_u64()?;
    let network_type = NetworkType::try_from(network)?;
    Ok(crate::utils::sompi_to_entropyx_string_with_suffix(sompi, &network_type))
}
