use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token::{self, Mint, Token};

use crate::constants::{LP_DECIMALS, MARKET_SEED, POOL_SEED, VAULT_SEED};
use crate::error::PerpsError;
use crate::state::{GlobalState, LiquidityPool, Market};

#[derive(Accounts)]
#[instruction(market_id: u16)]
pub struct CreateMarket<'info> {
    #[account(mut, seeds = [crate::constants::GLOBAL_STATE_SEED], bump = global_state.bump, has_one = admin)]
    pub global_state: Account<'info, GlobalState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + Market::INIT_SPACE,
        seeds = [MARKET_SEED, &market_id.to_le_bytes()],
        bump
    )]
    pub market: Account<'info, Market>,
    #[account(
        init,
        payer = admin,
        space = 8 + LiquidityPool::INIT_SPACE,
        seeds = [POOL_SEED, &market_id.to_le_bytes()],
        bump
    )]
    pub pool: Account<'info, LiquidityPool>,
    #[account(
        mut,
        seeds = [POOL_SEED, &market_id.to_le_bytes(), b"mint"],
        bump
    )]
    pub lp_mint: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [VAULT_SEED, &market_id.to_le_bytes()],
        bump
    )]
    pub vault: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<CreateMarket>,
    market_id: u16,
    oracle: Pubkey,
    max_leverage: u8,
    maintenance_margin_bps: u16,
    open_fee_bps: u16,
    close_fee_bps: u16,
    liquidation_fee_bps: u16,
    max_staleness_seconds: i64,
    max_confidence_bps: u64,
) -> Result<()> {
    require!(max_leverage > 0, PerpsError::InvalidLeverage);

    let market = &mut ctx.accounts.market;
    market.market_id = market_id;
    market.oracle = oracle;
    market.pool = ctx.accounts.pool.key();
    market.max_leverage = max_leverage;
    market.maintenance_margin_bps = maintenance_margin_bps;
    market.open_fee_bps = open_fee_bps;
    market.close_fee_bps = close_fee_bps;
    market.liquidation_fee_bps = liquidation_fee_bps;
    market.max_staleness_seconds = max_staleness_seconds;
    market.max_confidence_bps = max_confidence_bps;
    market.paused = false;
    market.bump = ctx.bumps.market;

    let pool = &mut ctx.accounts.pool;
    pool.market_id = market_id;
    pool.vault = ctx.accounts.vault.key();
    pool.lp_mint = ctx.accounts.lp_mint.key();
    pool.total_liquidity = 0;
    pool.total_lp_shares = 0;
    pool.locked_collateral = 0;
    pool.bump = ctx.bumps.pool;

    let rent = Rent::get()?;
    let vault_lamports = rent.minimum_balance(0);
    invoke_signed(
        &system_instruction::create_account(
            &ctx.accounts.admin.key(),
            &ctx.accounts.vault.key(),
            vault_lamports,
            0,
            &anchor_lang::solana_program::system_program::ID,
        ),
        &[
            ctx.accounts.admin.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[&[VAULT_SEED, &market_id.to_le_bytes(), &[ctx.bumps.vault]]],
    )?;

    let mint_lamports = rent.minimum_balance(Mint::LEN);
    invoke_signed(
        &system_instruction::create_account(
            &ctx.accounts.admin.key(),
            &ctx.accounts.lp_mint.key(),
            mint_lamports,
            Mint::LEN as u64,
            &anchor_spl::token::ID,
        ),
        &[
            ctx.accounts.admin.to_account_info(),
            ctx.accounts.lp_mint.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[&[POOL_SEED, &market_id.to_le_bytes(), b"mint", &[ctx.bumps.lp_mint]]],
    )?;

    token::initialize_mint(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::InitializeMint {
                mint: ctx.accounts.lp_mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        LP_DECIMALS,
        &ctx.accounts.pool.key(),
        Some(&ctx.accounts.pool.key()),
    )?;

    Ok(())
}
