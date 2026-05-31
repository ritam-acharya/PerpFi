use anchor_lang::prelude::*;

#[error_code]
pub enum PerpsError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Protocol is paused")]
    ProtocolPaused,
    #[msg("Market is paused")]
    MarketPaused,
    #[msg("Invalid market")]
    InvalidMarket,
    #[msg("Invalid oracle")]
    InvalidOracle,
    #[msg("Oracle price is stale")]
    StaleOracle,
    #[msg("Oracle confidence interval is too wide")]
    InvalidConfidence,
    #[msg("Oracle price must be positive")]
    InvalidPrice,
    #[msg("Invalid leverage")]
    InvalidLeverage,
    #[msg("Invalid collateral")]
    InvalidCollateral,
    #[msg("Position already exists")]
    PositionAlreadyExists,
    #[msg("Position not active")]
    PositionNotActive,
    #[msg("Position is healthy")]
    PositionHealthy,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Insufficient shares")]
    InsufficientShares,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Arithmetic underflow")]
    MathUnderflow,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("LP mint authority mismatch")]
    InvalidMintAuthority,
    #[msg("Invalid treasury")]
    InvalidTreasury,
    #[msg("Invalid owner")]
    InvalidOwner,
    #[msg("Invalid liquidation state")]
    InvalidLiquidationState,
}
