use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

use crate::constants::{POOL_SEED, VAULT_SEED};
use crate::error::PerpsError;
use crate::state::{GlobalState, LiquidityPool, Market};

#[derive(Accounts)]
#[instruction(market_id: u16)]
pub struct AddLiquidity<'info> {
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
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = lp_mint,
        associated_token::authority = user
    )]
    pub user_lp_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<AddLiquidity>, market_id: u16, amount: u64) -> Result<()> {
    require!(amount > 0, PerpsError::InvalidAmount);
    require!(ctx.accounts.market.market_id == market_id, PerpsError::InvalidMarket);
    require_keys_eq!(ctx.accounts.pool.lp_mint, ctx.accounts.lp_mint.key(), PerpsError::InvalidMarket);
    require_keys_eq!(ctx.accounts.pool.vault, ctx.accounts.vault.key(), PerpsError::InvalidMarket);
    require!(!ctx.accounts.market.paused, PerpsError::MarketPaused);

    let pool = &mut ctx.accounts.pool;
    let shares = if pool.total_lp_shares == 0 || pool.total_liquidity == 0 {
        amount
    } else {
        (amount as u128)
            .checked_mul(pool.total_lp_shares as u128)
            .ok_or_else(|| error!(PerpsError::MathOverflow))?
            .checked_div(pool.total_liquidity as u128)
            .ok_or_else(|| error!(PerpsError::MathOverflow))? as u64
    };
    require!(shares > 0, PerpsError::InvalidAmount);

    invoke(
        &anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.vault.key(),
            amount,
        ),
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    pool.total_liquidity = pool
        .total_liquidity
        .checked_add(amount)
        .ok_or_else(|| error!(PerpsError::MathOverflow))?;
    pool.total_lp_shares = pool
        .total_lp_shares
        .checked_add(shares)
        .ok_or_else(|| error!(PerpsError::MathOverflow))?;

    let signer_seeds: &[&[u8]] = &[
        POOL_SEED,
        &market_id.to_le_bytes(),
        &[pool.bump],
    ];
    let signer = &[signer_seeds];
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.lp_mint.to_account_info(),
                to: ctx.accounts.user_lp_ata.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
            },
            signer,
        ),
        shares,
    )?;

    Ok(())
}
