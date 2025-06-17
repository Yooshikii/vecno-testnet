use crate::result::Result;
use js_sys::BigInt;
use vecno_consensus_core::network::{NetworkType, NetworkTypeT};
use wasm_bindgen::prelude::*;
use workflow_wasm::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "bigint | number | HexString")]
    #[derive(Clone, Debug)]
    pub type ISompiToVecno;
}

/// Convert a Vecno string to Sompi represented by bigint.
/// This function provides correct precision handling and
/// can be used to parse user input.
/// @category Wallet SDK
#[wasm_bindgen(js_name = "vecnoToSompi")]
pub fn vecno_to_sompi(vecno: String) -> Option<BigInt> {
    crate::utils::try_vecno_str_to_sompi(vecno).ok().flatten().map(Into::into)
}

///
/// Convert Sompi to a string representation of the amount in Vecno.
///
/// @category Wallet SDK
///
#[wasm_bindgen(js_name = "sompiToVecnoString")]
pub fn sompi_to_vecno_string(sompi: ISompiToVecno) -> Result<String> {
    let sompi = sompi.try_as_u64()?;
    Ok(crate::utils::sompi_to_vecno_string(sompi))
}

///
/// Format a Sompi amount to a string representation of the amount in Vecno with a suffix
/// based on the network type (e.g. `VE` for mainnet, `TVE` for testnet,
/// `SVE` for simnet, `DVE` for devnet).
///
/// @category Wallet SDK
///
#[wasm_bindgen(js_name = "sompiToVecnoStringWithSuffix")]
pub fn sompi_to_vecno_string_with_suffix(sompi: ISompiToVecno, network: &NetworkTypeT) -> Result<String> {
    let sompi = sompi.try_as_u64()?;
    let network_type = NetworkType::try_from(network)?;
    Ok(crate::utils::sompi_to_vecno_string_with_suffix(sompi, &network_type))
}
