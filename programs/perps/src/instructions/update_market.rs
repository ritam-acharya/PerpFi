use anchor_lang::prelude::*;

use crate::error::PerpsError;
use crate::state::{GlobalState, Market};

#[derive(Accounts)]
#[instruction(market_id: u16)]
pub struct UpdateMarket<'info> {
    #[account(seeds = [crate::constants::GLOBAL_STATE_SEED], bump = global_state.bump)]
    pub global_state: Account<'info, GlobalState>,
    pub admin: Signer<'info>,
    #[account(mut, seeds = [crate::constants::MARKET_SEED, &market_id.to_le_bytes()], bump = market.bump)]
    pub market: Account<'info, Market>,
}

pub fn handler(
    ctx: Context<UpdateMarket>,
    oracle: Option<Pubkey>,
    max_leverage: Option<u8>,
    maintenance_margin_bps: Option<u16>,
    open_fee_bps: Option<u16>,
    close_fee_bps: Option<u16>,
    liquidation_fee_bps: Option<u16>,
    max_staleness_seconds: Option<i64>,
    max_confidence_bps: Option<u64>,
) -> Result<()> {
    require_keys_eq!(ctx.accounts.admin.key(), ctx.accounts.global_state.admin, PerpsError::Unauthorized);
    let market = &mut ctx.accounts.market;
    if let Some(value) = oracle {
        market.oracle = value;
    }
    if let Some(value) = max_leverage {
        market.max_leverage = value;
    }
    if let Some(value) = maintenance_margin_bps {
        market.maintenance_margin_bps = value;
    }
    if let Some(value) = open_fee_bps {
        market.open_fee_bps = value;
    }
    if let Some(value) = close_fee_bps {
        market.close_fee_bps = value;
    }
    if let Some(value) = liquidation_fee_bps {
        market.liquidation_fee_bps = value;
    }
    if let Some(value) = max_staleness_seconds {
        market.max_staleness_seconds = value;
    }
    if let Some(value) = max_confidence_bps {
        market.max_confidence_bps = value;
    }
    Ok(())
}
