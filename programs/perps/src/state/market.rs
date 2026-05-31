use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub market_id: u16,
    pub oracle: Pubkey,
    pub pool: Pubkey,
    pub max_leverage: u8,
    pub maintenance_margin_bps: u16,
    pub open_fee_bps: u16,
    pub close_fee_bps: u16,
    pub liquidation_fee_bps: u16,
    pub max_staleness_seconds: i64,
    pub max_confidence_bps: u64,
    pub paused: bool,
    pub bump: u8,
}
