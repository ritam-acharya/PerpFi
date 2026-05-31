import * as anchor from '@project-serum/anchor';
import idl from './perps_idl.json';
import { PublicKey } from '@solana/web3.js';

export function getProgram(provider: anchor.AnchorProvider, programId: string | PublicKey) {
  const pid = typeof programId === 'string' ? new PublicKey(programId) : programId;
  return new anchor.Program(idl as any, pid, provider);
}

export async function createMarket(
  provider: anchor.AnchorProvider,
  programId: string | PublicKey,
  params: {
    market_id: number;
    oracle: PublicKey;
    max_leverage: number;
    maintenance_margin_bps: number;
    open_fee_bps: number;
    close_fee_bps: number;
    liquidation_fee_bps: number;
    max_staleness_seconds: number;
    max_confidence_bps: number | string;
  },
  accounts: {
    globalState: PublicKey;
    admin: PublicKey;
    market: PublicKey;
    pool: PublicKey;
    lpMint: PublicKey;
    vault: PublicKey;
    tokenProgram: PublicKey;
    systemProgram: PublicKey;
    rent: PublicKey;
  }
) {
  const program = getProgram(provider, programId);
  return program.methods
    .createMarket(
      params.market_id,
      params.oracle,
      params.max_leverage,
      params.maintenance_margin_bps,
      params.open_fee_bps,
      params.close_fee_bps,
      params.liquidation_fee_bps,
      new anchor.BN(params.max_staleness_seconds),
      new anchor.BN(params.max_confidence_bps)
    )
    .accounts({
      globalState: accounts.globalState,
      admin: accounts.admin,
      market: accounts.market,
      pool: accounts.pool,
      lpMint: accounts.lpMint,
      vault: accounts.vault,
      tokenProgram: accounts.tokenProgram,
      systemProgram: accounts.systemProgram,
      rent: accounts.rent,
    })
    .rpc();
}

export async function addLiquidity(
  provider: anchor.AnchorProvider,
  programId: string | PublicKey,
  market_id: number,
  amount: number,
  accounts: any
) {
  const program = getProgram(provider, programId);
  return program.methods.addLiquidity(market_id, new anchor.BN(amount)).accounts(accounts).rpc();
}

// More wrappers can be added similarly for openPosition, closePosition, etc.

export default { getProgram, createMarket, addLiquidity };
