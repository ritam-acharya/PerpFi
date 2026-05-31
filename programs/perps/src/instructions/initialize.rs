use anchor_lang::prelude::*;

use crate::constants::{DEFAULT_MAX_CONFIDENCE_BPS, DEFAULT_MAX_STALENESS_SECONDS, TREASURY_SEED, GLOBAL_STATE_SEED};
use crate::state::{GlobalState, Treasury};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + GlobalState::INIT_SPACE,
        seeds = [GLOBAL_STATE_SEED],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        init,
        payer = admin,
        space = 8 + Treasury::INIT_SPACE,
        seeds = [TREASURY_SEED],
        bump
    )]
    pub treasury: Account<'info, Treasury>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;
    global_state.admin = ctx.accounts.admin.key();
    global_state.treasury = ctx.accounts.treasury.key();
    global_state.max_staleness_seconds = DEFAULT_MAX_STALENESS_SECONDS;
    global_state.max_confidence_bps = DEFAULT_MAX_CONFIDENCE_BPS;
    global_state.bump = ctx.bumps.global_state;

    let treasury = &mut ctx.accounts.treasury;
    treasury.authority = ctx.accounts.admin.key();
    treasury.collected_fees = 0;
    treasury.bump = ctx.bumps.treasury;
    Ok(())
}
