use anchor_lang::prelude::*;
use anchor_lang::{Accounts, AnchorDeserialize, AnchorSerialize, Bumps};
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::program_error::ProgramError;
use anchor_lang::solana_program::pubkey::Pubkey;
use std::collections::BTreeSet;

pub mod constants;
pub mod error;
pub mod math;
pub mod state;

#[path = "instructions/mod.rs"]
pub mod ix;

pub use ix::{
    add_liquidity::AddLiquidity,
    close_position::ClosePosition,
    create_market::CreateMarket,
    initialize::Initialize,
    liquidate::Liquidate,
    open_position::OpenPosition,
    pause_market::PauseMarket,
    remove_liquidity::RemoveLiquidity,
    update_market::UpdateMarket,
};

pub use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWqkZ7o6D4VQKxxqA3hS3hM1");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum PerpsInstruction {
    Initialize,
    CreateMarket {
        market_id: u16,
        oracle: Pubkey,
        max_leverage: u8,
        maintenance_margin_bps: u16,
        open_fee_bps: u16,
        close_fee_bps: u16,
        liquidation_fee_bps: u16,
        max_staleness_seconds: i64,
        max_confidence_bps: u64,
    },
    AddLiquidity { market_id: u16, amount: u64 },
    RemoveLiquidity { market_id: u16, shares: u64 },
    OpenPosition {
        market_id: u16,
        side: Side,
        collateral: u64,
        position_size: u64,
        leverage: u8,
    },
    ClosePosition { market_id: u16 },
    Liquidate { market_id: u16 },
    PauseMarket { market_id: u16, paused: bool },
    UpdateMarket {
        market_id: u16,
        oracle: Option<Pubkey>,
        max_leverage: Option<u8>,
        maintenance_margin_bps: Option<u16>,
        open_fee_bps: Option<u16>,
        close_fee_bps: Option<u16>,
        liquidation_fee_bps: Option<u16>,
        max_staleness_seconds: Option<i64>,
        max_confidence_bps: Option<u64>,
    },
}

pub fn process_instruction<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = PerpsInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        PerpsInstruction::Initialize => {
            dispatch_with_args::<Initialize, _>(program_id, accounts, &[], ix::initialize::handler)
        }
        PerpsInstruction::CreateMarket {
            market_id,
            oracle,
            max_leverage,
            maintenance_margin_bps,
            open_fee_bps,
            close_fee_bps,
            liquidation_fee_bps,
            max_staleness_seconds,
            max_confidence_bps,
        } => dispatch_with_args::<CreateMarket, _>(
            program_id,
            accounts,
            &market_id.to_le_bytes(),
            |ctx| {
                ix::create_market::handler(
                    ctx,
                    market_id,
                    oracle,
                    max_leverage,
                    maintenance_margin_bps,
                    open_fee_bps,
                    close_fee_bps,
                    liquidation_fee_bps,
                    max_staleness_seconds,
                    max_confidence_bps,
                )
            },
        ),
        PerpsInstruction::AddLiquidity { market_id, amount } => dispatch_with_args::<AddLiquidity, _>(
            program_id,
            accounts,
            &market_id.to_le_bytes(),
            |ctx| ix::add_liquidity::handler(ctx, market_id, amount),
        ),
        PerpsInstruction::RemoveLiquidity { market_id, shares } => dispatch_with_args::<RemoveLiquidity, _>(
            program_id,
            accounts,
            &market_id.to_le_bytes(),
            |ctx| ix::remove_liquidity::handler(ctx, market_id, shares),
        ),
        PerpsInstruction::OpenPosition {
            market_id,
            side,
            collateral,
            position_size,
            leverage,
        } => dispatch_with_args::<OpenPosition, _>(
            program_id,
            accounts,
            &market_id.to_le_bytes(),
            |ctx| ix::open_position::handler(ctx, market_id, side, collateral, position_size, leverage),
        ),
        PerpsInstruction::ClosePosition { market_id } => dispatch_with_args::<ClosePosition, _>(
            program_id,
            accounts,
            &market_id.to_le_bytes(),
            |ctx| ix::close_position::handler(ctx, market_id),
        ),
        PerpsInstruction::Liquidate { market_id } => dispatch_with_args::<Liquidate, _>(
            program_id,
            accounts,
            &market_id.to_le_bytes(),
            |ctx| ix::liquidate::handler(ctx, market_id),
        ),
        PerpsInstruction::PauseMarket { market_id, paused } => dispatch_with_args::<PauseMarket, _>(
            program_id,
            accounts,
            &market_id.to_le_bytes(),
            |ctx| ix::pause_market::handler(ctx, paused),
        ),
        PerpsInstruction::UpdateMarket {
            market_id,
            oracle,
            max_leverage,
            maintenance_margin_bps,
            open_fee_bps,
            close_fee_bps,
            liquidation_fee_bps,
            max_staleness_seconds,
            max_confidence_bps,
        } => dispatch_with_args::<UpdateMarket, _>(
            program_id,
            accounts,
            &market_id.to_le_bytes(),
            |ctx| {
                ix::update_market::handler(
                    ctx,
                    oracle,
                    max_leverage,
                    maintenance_margin_bps,
                    open_fee_bps,
                    close_fee_bps,
                    liquidation_fee_bps,
                    max_staleness_seconds,
                    max_confidence_bps,
                )
            },
        ),
    }
}

fn dispatch_with_args<'info, A, F>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    ix_data: &[u8],
    handler: F,
) -> ProgramResult
where
    A: Accounts<'info, <A as Bumps>::Bumps> + Bumps,
    <A as Bumps>::Bumps: Default,
    F: FnOnce(Context<'_, '_, '_, 'info, A>) -> Result<()>,
{
    let mut remaining_accounts = accounts;
    let mut bumps = <A as Bumps>::Bumps::default();
    let mut reallocs = BTreeSet::new();
    let mut validated_accounts = A::try_accounts(
        program_id,
        &mut remaining_accounts,
        ix_data,
        &mut bumps,
        &mut reallocs,
    )
    .map_err(ProgramError::from)?;
    let ctx = Context::new(program_id, &mut validated_accounts, remaining_accounts, bumps);
    handler(ctx).map_err(Into::into)
}
