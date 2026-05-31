use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;

use crate::constants::{BASIS_POINTS_DIVISOR, MIN_COLLATERAL_LAMPORTS, POSITION_SEED, VAULT_SEED};
use crate::error::PerpsError;
use crate::math::load_price_with_checks;
use crate::state::{GlobalState, LiquidityPool, Market, Position, Side, Treasury};

#[derive(Accounts)]
#[instruction(market_id: u16)]
pub struct OpenPosition<'info> {
    #[account(seeds = [crate::constants::GLOBAL_STATE_SEED], bump = global_state.bump)]
    pub global_state: Account<'info, GlobalState>,
    #[account(mut, seeds = [crate::constants::TREASURY_SEED], bump = treasury.bump)]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [crate::constants::MARKET_SEED, &market_id.to_le_bytes()], bump = market.bump)]
    pub market: Account<'info, Market>,
    #[account(mut, seeds = [crate::constants::POOL_SEED, &market_id.to_le_bytes()], bump = pool.bump)]
    pub pool: Account<'info, LiquidityPool>,
    #[account(mut, seeds = [VAULT_SEED, &market_id.to_le_bytes()], bump)]
    pub vault: SystemAccount<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + Position::INIT_SPACE,
        seeds = [POSITION_SEED, user.key().as_ref(), &market_id.to_le_bytes()],
        bump
    )]
    pub position: Account<'info, Position>,
    /// CHECK: validated via market.oracle.
    pub oracle: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<OpenPosition>,
    market_id: u16,
    side: Side,
    collateral: u64,
    position_size: u64,
    leverage: u8,
) -> Result<()> {
    require!(collateral >= MIN_COLLATERAL_LAMPORTS, PerpsError::InvalidCollateral);
    require!(position_size > 0, PerpsError::InvalidAmount);
    require!(leverage > 0 && leverage <= ctx.accounts.market.max_leverage, PerpsError::InvalidLeverage);
    require!(!ctx.accounts.market.paused, PerpsError::MarketPaused);
    require!(ctx.accounts.market.market_id == market_id, PerpsError::InvalidMarket);
    require_keys_eq!(ctx.accounts.market.oracle, ctx.accounts.oracle.key(), PerpsError::InvalidOracle);
    let current_price = load_price_with_checks(
        &ctx.accounts.oracle,
        ctx.accounts.market.max_staleness_seconds.min(ctx.accounts.global_state.max_staleness_seconds),
        ctx.accounts.market.max_confidence_bps.min(ctx.accounts.global_state.max_confidence_bps),
    )?;

    invoke(
        &anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.vault.key(),
            collateral,
        ),
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    let open_fee = (collateral as u128)
        .checked_mul(ctx.accounts.market.open_fee_bps as u128)
        .ok_or_else(|| error!(PerpsError::MathOverflow))?
        / BASIS_POINTS_DIVISOR as u128;
    if open_fee > 0 {
        invoke(
            &anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.vault.key(),
                &ctx.accounts.treasury.key(),
                open_fee as u64,
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
            .checked_add(open_fee as u64)
            .ok_or_else(|| error!(PerpsError::MathOverflow))?;
    }

    let net_collateral = collateral.checked_sub(open_fee as u64).ok_or_else(|| error!(PerpsError::MathUnderflow))?;
    let required_margin = (position_size as u128)
        .checked_div(leverage as u128)
        .ok_or_else(|| error!(PerpsError::MathOverflow))? as u64;
    require!(net_collateral >= required_margin, PerpsError::InvalidCollateral);

    let position = &mut ctx.accounts.position;
    position.owner = ctx.accounts.user.key();
    position.market_id = market_id;
    position.side = side;
    position.collateral = net_collateral;
    position.position_size = position_size;
    position.leverage = leverage;
    position.entry_price = current_price;
    position.realized_pnl = 0;
    position.opened_at = Clock::get()?.unix_timestamp;
    position.bump = ctx.bumps.position;

    let pool = &mut ctx.accounts.pool;
    pool.locked_collateral = pool
        .locked_collateral
        .checked_add(net_collateral)
        .ok_or_else(|| error!(PerpsError::MathOverflow))?;

    Ok(())
}
