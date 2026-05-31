pub mod fees;
pub mod liquidation;
pub mod pnl;

use anchor_lang::prelude::*;
use pyth_sdk_solana::load_price_feed_from_account_info;

use crate::error::PerpsError;

pub fn load_price_with_checks(
    price_account_info: &AccountInfo,
    max_staleness_seconds: i64,
    max_confidence_bps: u64,
) -> Result<i64> {
    let price_feed = load_price_feed_from_account_info(price_account_info)
        .map_err(|_| error!(PerpsError::InvalidOracle))?;
    let clock = Clock::get()?;
    let current_price = price_feed
        .get_price_no_older_than(clock.unix_timestamp, max_staleness_seconds as u64)
        .ok_or_else(|| error!(PerpsError::StaleOracle))?;

    require!(current_price.price > 0, PerpsError::InvalidPrice);

    let price_abs = current_price.price.unsigned_abs() as u128;
    let conf = current_price.conf as u128;
    require!(price_abs > 0, PerpsError::InvalidPrice);
    require!(
        conf.saturating_mul(10_000) <= price_abs.saturating_mul(max_confidence_bps as u128),
        PerpsError::InvalidConfidence
    );

    Ok(current_price.price)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Side;

    #[test]
    fn calculates_fee_in_basis_points() {
        assert_eq!(fees::fee(10_000, 25).unwrap(), 25);
    }

    #[test]
    fn calculates_long_and_short_pnl() {
        assert_eq!(pnl::calculate_pnl(Side::Long, 50, 100, 120).unwrap(), 10);
        assert_eq!(pnl::calculate_pnl(Side::Short, 50, 120, 100).unwrap(), 8);
    }

    #[test]
    fn equity_is_saturating_on_loss() {
        assert_eq!(pnl::equity_after_pnl(100, -150).unwrap(), 0);
        assert_eq!(pnl::equity_after_pnl(100, 25).unwrap(), 125);
    }

    #[test]
    fn liquidation_math_scales_linearly() {
        assert_eq!(liquidation::maintenance_requirement(10_000, 500).unwrap(), 500);
        assert_eq!(liquidation::margin_ratio_bps(500, 10_000).unwrap(), 500);
    }
}
