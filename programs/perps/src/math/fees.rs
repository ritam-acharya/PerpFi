use anchor_lang::prelude::*;

use crate::error::PerpsError;
use crate::constants::BASIS_POINTS_DIVISOR;

pub fn fee(amount: u64, fee_bps: u16) -> Result<u64> {
    let numerator = (amount as u128)
        .checked_mul(fee_bps as u128)
        .ok_or_else(|| error!(PerpsError::MathOverflow))?;
    Ok((numerator / BASIS_POINTS_DIVISOR as u128) as u64)
}
