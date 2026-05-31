import React from 'react';
import * as anchor from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { getProgram, createMarket } from '../perps';

// Example React snippet using an Anchor Provider (with a connected wallet)
export function PerpsExample({ provider, programId }: { provider: anchor.AnchorProvider; programId: string; }) {
  const onCreateMarket = async () => {
    const params = {
      market_id: 1,
      oracle: new PublicKey('EnterPythPriceAcctPubkeyHere'),
      max_leverage: 10,
      maintenance_margin_bps: 500,
      open_fee_bps: 10,
      close_fee_bps: 10,
      liquidation_fee_bps: 100,
      max_staleness_seconds: 60,
      max_confidence_bps: 1000,
    };
    // Derive PDAs client-side using the same seeds from the program constants
    // Replace these with your derived PDAs
    const accounts = {
      globalState: new PublicKey('ReplaceWithGlobalStatePubkey'),
      admin: provider.wallet.publicKey,
      market: new PublicKey('ReplaceWithMarketPubkey'),
      pool: new PublicKey('ReplaceWithPoolPubkey'),
      lpMint: new PublicKey('ReplaceWithLpMintPubkey'),
      vault: new PublicKey('ReplaceWithVaultPubkey'),
      tokenProgram: anchor.web3.TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    };

    try {
      await createMarket(provider, programId, params, accounts as any);
      alert('createMarket tx sent');
    } catch (err) {
      console.error('createMarket error', err);
      alert('error: ' + String(err));
    }
  };

  return (
    <div>
      <button onClick={onCreateMarket}>Create Market</button>
    </div>
  );
}
