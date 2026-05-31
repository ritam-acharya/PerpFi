use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};

use crate::constants::{POOL_SEED, VAULT_SEED};
use crate::error::PerpsError;
use crate::state::{GlobalState, LiquidityPool, Market};

#[derive(Accounts)]
#[instruction(market_id: u16)]
pub struct RemoveLiquidity<'info> {
    #[account(seeds = [crate::constants::GLOBAL_STATE_SEED], bump = global_state.bump)]
    pub global_state: Account<'info, GlobalState>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [crate::constants::MARKET_SEED, &market_id.to_le_bytes()], bump = market.bump)]
    pub market: Account<'info, Market>,
    #[account(mut, seeds = [POOL_SEED, &market_id.to_le_bytes()], bump = pool.bump)]
    pub pool: Account<'info, LiquidityPool>,
    #[account(mut, seeds = [VAULT_SEED, &market_id.to_le_bytes()], bump)]
    pub vault: SystemAccount<'info>,
    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_lp_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RemoveLiquidity>, market_id: u16, shares: u64) -> Result<()> {
    require!(shares > 0, PerpsError::InvalidAmount);
    require!(!ctx.accounts.market.paused, PerpsError::MarketPaused);
    require!(ctx.accounts.market.market_id == market_id, PerpsError::InvalidMarket);
    require_keys_eq!(ctx.accounts.pool.lp_mint, ctx.accounts.lp_mint.key(), PerpsError::InvalidMarket);
    require_keys_eq!(ctx.accounts.pool.vault, ctx.accounts.vault.key(), PerpsError::InvalidMarket);
    require_keys_eq!(ctx.accounts.user_lp_ata.owner, ctx.accounts.user.key(), PerpsError::InvalidOwner);
    require!(ctx.accounts.pool.total_lp_shares >= shares, PerpsError::InsufficientShares);

    let pool = &mut ctx.accounts.pool;
    let withdraw_amount = (shares as u128)
        .checked_mul(pool.total_liquidity as u128)
        .ok_or_else(|| error!(PerpsError::MathOverflow))?
        .checked_div(pool.total_lp_shares as u128)
        .ok_or_else(|| error!(PerpsError::MathOverflow))? as u64;

    let free_liquidity = ctx.accounts.vault.to_account_info().lamports();
    require!(free_liquidity >= pool.locked_collateral, PerpsError::InsufficientLiquidity);
    require!(withdraw_amount <= free_liquidity - pool.locked_collateral, PerpsError::InsufficientLiquidity);

    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.lp_mint.to_account_info(),
                from: ctx.accounts.user_lp_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        shares,
    )?;

    pool.total_lp_shares = pool
        .total_lp_shares
        .checked_sub(shares)
        .ok_or_else(|| error!(PerpsError::MathUnderflow))?;
    pool.total_liquidity = pool
        .total_liquidity
        .checked_sub(withdraw_amount)
        .ok_or_else(|| error!(PerpsError::MathUnderflow))?;

    invoke(
        &anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.vault.key(),
            &ctx.accounts.user.key(),
            withdraw_amount,
        ),
        &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    Ok(())
}
