use crate::error::Error;
use crate::result::Result;
use vecno_consensus_core::constants::SOMPI_PER_VECNO;
use std::fmt::Display;

pub fn try_parse_required_nonzero_vecno_as_sompi_u64<S: ToString + Display>(vecno_amount: Option<S>) -> Result<u64> {
    if let Some(vecno_amount) = vecno_amount {
        let sompi_amount = vecno_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Vecno amount is not valid: '{vecno_amount}'")))?
            * SOMPI_PER_VECNO as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Vecno amount is not valid: '{vecno_amount}'"))
        } else {
            let sompi_amount = sompi_amount as u64;
            if sompi_amount == 0 {
                Err(Error::custom("Supplied required vecno amount must not be a zero: '{vecno_amount}'"))
            } else {
                Ok(sompi_amount)
            }
        }
    } else {
        Err(Error::custom("Missing Vecno amount"))
    }
}

pub fn try_parse_required_vecno_as_sompi_u64<S: ToString + Display>(vecno_amount: Option<S>) -> Result<u64> {
    if let Some(vecno_amount) = vecno_amount {
        let sompi_amount = vecno_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Vecno amount is not valid: '{vecno_amount}'")))?
            * SOMPI_PER_VECNO as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Vecno amount is not valid: '{vecno_amount}'"))
        } else {
            Ok(sompi_amount as u64)
        }
    } else {
        Err(Error::custom("Missing Vecno amount"))
    }
}

pub fn try_parse_optional_vecno_as_sompi_i64<S: ToString + Display>(vecno_amount: Option<S>) -> Result<Option<i64>> {
    if let Some(vecno_amount) = vecno_amount {
        let sompi_amount = vecno_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_e| Error::custom(format!("Supplied Vecno amount is not valid: '{vecno_amount}'")))?
            * SOMPI_PER_VECNO as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Vecno amount is not valid: '{vecno_amount}'"))
        } else {
            Ok(Some(sompi_amount as i64))
        }
    } else {
        Ok(None)
    }
}
