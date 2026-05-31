use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Treasury {
    pub authority: Pubkey,
    pub collected_fees: u64,
    pub bump: u8,
}
