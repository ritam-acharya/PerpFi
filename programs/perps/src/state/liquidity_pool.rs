use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct LiquidityPool {
    pub market_id: u16,
    pub vault: Pubkey,
    pub lp_mint: Pubkey,
    pub total_liquidity: u64,
    pub total_lp_shares: u64,
    pub locked_collateral: u64,
    pub bump: u8,
}
