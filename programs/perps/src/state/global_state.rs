use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GlobalState {
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub max_staleness_seconds: i64,
    pub max_confidence_bps: u64,
    pub bump: u8,
}
