use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;

use crate::constants::{POSITION_SEED, VAULT_SEED};
use crate::error::PerpsError;
use crate::math::{fees::fee, load_price_with_checks, pnl::{calculate_pnl, equity_after_pnl}};
use crate::state::{GlobalState, LiquidityPool, Market, Position};

#[derive(Accounts)]
#[instruction(market_id: u16)]
pub struct ClosePosition<'info> {
    #[account(seeds = [crate::constants::GLOBAL_STATE_SEED], bump = global_state.bump)]
    pub global_state: Account<'info, GlobalState>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [crate::constants::MARKET_SEED, &market_id.to_le_bytes()], bump = market.bump)]
    pub market: Account<'info, Market>,
    #[account(mut, seeds = [crate::constants::POOL_SEED, &market_id.to_le_bytes()], bump = pool.bump)]
    pub pool: Account<'info, LiquidityPool>,
    #[account(mut, seeds = [VAULT_SEED, &market_id.to_le_bytes()], bump)]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        close = user,
        seeds = [POSITION_SEED, user.key().as_ref(), &market_id.to_le_bytes()],
        bump = position.bump,
        has_one = owner
    )]
    pub position: Account<'info, Position>,
    /// CHECK: bound by `has_one = owner`.
    pub owner: UncheckedAccount<'info>,
    /// CHECK: validated via market.oracle.
    pub oracle: AccountInfo<'info>,
    #[account(mut, seeds = [crate::constants::TREASURY_SEED], bump = treasury.bump)]
    pub treasury: Account<'info, crate::state::Treasury>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ClosePosition>, market_id: u16) -> Result<()> {
    require!(!ctx.accounts.market.paused, PerpsError::MarketPaused);
    require!(ctx.accounts.market.market_id == market_id, PerpsError::InvalidMarket);
    require_keys_eq!(ctx.accounts.market.oracle, ctx.accounts.oracle.key(), PerpsError::InvalidOracle);
    require_keys_eq!(ctx.accounts.position.owner, ctx.accounts.user.key(), PerpsError::InvalidOwner);

    let current_price = load_price_with_checks(
        &ctx.accounts.oracle,
        ctx.accounts.market.max_staleness_seconds.min(ctx.accounts.global_state.max_staleness_seconds),
        ctx.accounts.market.max_confidence_bps.min(ctx.accounts.global_state.max_confidence_bps),
    )?;

    let pnl = calculate_pnl(
        ctx.accounts.position.side,
        ctx.accounts.position.position_size,
        ctx.accounts.position.entry_price,
        current_price,
    )?;
    require!(
        pnl >= -(ctx.accounts.position.collateral as i64),
        PerpsError::PositionHealthy
    );
    let equity = equity_after_pnl(ctx.accounts.position.collateral, pnl)?;
    let close_fee = fee(equity, ctx.accounts.market.close_fee_bps)?;
    let payout = equity.saturating_sub(close_fee);

    let pool = &mut ctx.accounts.pool;
    pool.locked_collateral = pool
        .locked_collateral
        .checked_sub(ctx.accounts.position.collateral)
        .ok_or_else(|| error!(PerpsError::MathUnderflow))?;
    pool.total_liquidity = if pnl >= 0 {
        pool.total_liquidity.checked_sub(pnl as u64).ok_or_else(|| error!(PerpsError::MathUnderflow))?
    } else {
        pool.total_liquidity.checked_add(pnl.unsigned_abs()).ok_or_else(|| error!(PerpsError::MathOverflow))?
    };
    pool.total_liquidity = pool
        .total_liquidity
        .checked_sub(close_fee)
        .ok_or_else(|| error!(PerpsError::MathUnderflow))?;

    if close_fee > 0 {
        invoke(
            &anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.vault.key(),
                &ctx.accounts.treasury.key(),
                close_fee,
            ),
            &[
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        ctx.accounts.treasury.collected_fees = ctx
            .accounts
            .treasury
            .collected_fees
            .checked_add(close_fee)
            .ok_or_else(|| error!(PerpsError::MathOverflow))?;
    }

    if payout > 0 {
        invoke(
            &anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.vault.key(),
                &ctx.accounts.user.key(),
                payout,
            ),
            &[
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
    }

    Ok(())
}
