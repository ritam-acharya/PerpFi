use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
#[derive(Debug)]
pub enum Side {
    Long,
    Short,
}

#[account]
#[derive(InitSpace)]
pub struct Position {
    pub owner: Pubkey,
    pub market_id: u16,
    pub side: Side,
    pub collateral: u64,
    pub position_size: u64,
    pub leverage: u8,
    pub entry_price: i64,
    pub realized_pnl: i64,
    pub opened_at: i64,
    pub bump: u8,
}
