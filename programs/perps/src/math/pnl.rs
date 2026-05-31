use anchor_lang::prelude::*;

use crate::error::PerpsError;
use crate::state::Side;

pub fn calculate_pnl(side: Side, position_size: u64, entry_price: i64, current_price: i64) -> Result<i64> {
    require!(entry_price > 0, PerpsError::InvalidPrice);
    require!(current_price > 0, PerpsError::InvalidPrice);

    let entry = entry_price as i128;
    let current = current_price as i128;
    let size = position_size as i128;

    let delta = match side {
        Side::Long => current.checked_sub(entry).ok_or_else(|| error!(PerpsError::MathOverflow))?,
        Side::Short => entry.checked_sub(current).ok_or_else(|| error!(PerpsError::MathOverflow))?,
    };

    let pnl = delta
        .checked_mul(size)
        .ok_or_else(|| error!(PerpsError::MathOverflow))?
        .checked_div(entry)
        .ok_or_else(|| error!(PerpsError::MathOverflow))?;

    Ok(pnl as i64)
}

pub fn equity_after_pnl(collateral: u64, pnl: i64) -> Result<u64> {
    if pnl >= 0 {
        collateral
            .checked_add(pnl as u64)
            .ok_or_else(|| error!(PerpsError::MathOverflow))
    } else {
        Ok(collateral.saturating_sub(pnl.unsigned_abs()))
    }
}
