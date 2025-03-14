use crate::error::Error;
use crate::result::Result;
use entropyx_consensus_core::constants::SOMPI_PER_ENX;
use std::fmt::Display;

pub fn try_parse_required_nonzero_entropyx_as_sompi_u64<S: ToString + Display>(entropyx_amount: Option<S>) -> Result<u64> {
    if let Some(entropyx_amount) = entropyx_amount {
        let sompi_amount = entropyx_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Encapa amount is not valid: '{entropyx_amount}'")))?
            * SOMPI_PER_ENX as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied EntropyX amount is not valid: '{entropyx_amount}'"))
        } else {
            let sompi_amount = sompi_amount as u64;
            if sompi_amount == 0 {
                Err(Error::custom("Supplied required entropyx amount must not be a zero: '{entropyx_amount}'"))
            } else {
                Ok(sompi_amount)
            }
        }
    } else {
        Err(Error::custom("Missing EntropyX amount"))
    }
}

pub fn try_parse_required_entropyx_as_sompi_u64<S: ToString + Display>(entropyx_amount: Option<S>) -> Result<u64> {
    if let Some(entropyx_amount) = entropyx_amount {
        let sompi_amount = entropyx_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Encapa amount is not valid: '{entropyx_amount}'")))?
            * SOMPI_PER_ENX as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied EntropyX amount is not valid: '{entropyx_amount}'"))
        } else {
            Ok(sompi_amount as u64)
        }
    } else {
        Err(Error::custom("Missing EntropyX amount"))
    }
}

pub fn try_parse_optional_entropyx_as_sompi_i64<S: ToString + Display>(entropyx_amount: Option<S>) -> Result<Option<i64>> {
    if let Some(entropyx_amount) = entropyx_amount {
        let sompi_amount = entropyx_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_e| Error::custom(format!("Supplied Encapa amount is not valid: '{entropyx_amount}'")))?
            * SOMPI_PER_ENX as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied EntropyX amount is not valid: '{entropyx_amount}'"))
        } else {
            Ok(Some(sompi_amount as i64))
        }
    } else {
        Ok(None)
    }
}
