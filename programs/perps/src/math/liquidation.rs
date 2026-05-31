use anchor_lang::prelude::*;

use crate::constants::BASIS_POINTS_DIVISOR;
use crate::error::PerpsError;

pub fn maintenance_requirement(position_size: u64, maintenance_margin_bps: u16) -> Result<u64> {
    let requirement = (position_size as u128)
        .checked_mul(maintenance_margin_bps as u128)
        .ok_or_else(|| error!(PerpsError::MathOverflow))?
        / BASIS_POINTS_DIVISOR as u128;
    Ok(requirement as u64)
}

pub fn margin_ratio_bps(equity: u64, position_size: u64) -> Result<u64> {
    if position_size == 0 {
        return Ok(0);
    }
    let ratio = (equity as u128)
        .checked_mul(BASIS_POINTS_DIVISOR as u128)
        .ok_or_else(|| error!(PerpsError::MathOverflow))?
        / position_size as u128;
    Ok(ratio as u64)
}
