use anchor_lang::prelude::*;

use crate::error::PerpsError;
use crate::state::{GlobalState, Market};

#[derive(Accounts)]
#[instruction(market_id: u16)]
pub struct PauseMarket<'info> {
    #[account(seeds = [crate::constants::GLOBAL_STATE_SEED], bump = global_state.bump)]
    pub global_state: Account<'info, GlobalState>,
    pub admin: Signer<'info>,
    #[account(mut, seeds = [crate::constants::MARKET_SEED, &market_id.to_le_bytes()], bump = market.bump)]
    pub market: Account<'info, Market>,
}

pub fn handler(ctx: Context<PauseMarket>, paused: bool) -> Result<()> {
    require_keys_eq!(ctx.accounts.admin.key(), ctx.accounts.global_state.admin, PerpsError::Unauthorized);
    ctx.accounts.market.paused = paused;
    Ok(())
}
