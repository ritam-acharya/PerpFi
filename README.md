# Perps Protocol

A decentralized perpetual futures trading protocol built on Solana using Anchor Framework.

## Overview

Perps Protocol enables users to:

* Open leveraged long and short positions
* Provide liquidity to trading pools
* Earn trading fees as liquidity providers
* Trade using oracle-based pricing
* Liquidate undercollateralized positions
* Manage markets through configurable risk parameters

The protocol is fully on-chain and designed around Solana's high-performance execution environment.

---

## Features

### Trading

* Long positions
* Short positions
* Configurable leverage limits
* Oracle-based mark pricing
* Position PnL calculation
* Position closing
* Position liquidation

### Liquidity Provision

* Deposit liquidity into market pools
* Receive LP shares
* Withdraw liquidity
* Share in protocol trading fees

### Risk Management

* Maximum leverage enforcement
* Maintenance margin requirements
* Oracle confidence checks
* Oracle staleness protection
* Market pause functionality
* Liquidation engine

### Administration

* Protocol initialization
* Market creation
* Market parameter updates
* Market pause/unpause controls

---

# Architecture

## Core Accounts

### GlobalState

Stores protocol-wide configuration.

Fields include:

* Admin authority
* Maximum oracle staleness
* Maximum oracle confidence threshold
* Protocol bump seeds

---

### Market

Represents a perpetual futures market.

Fields include:

* Market ID
* Oracle address
* Liquidity pool reference
* Maximum leverage
* Maintenance margin
* Open fee
* Close fee
* Liquidation fee
* Oracle configuration
* Pause status

---

### LiquidityPool

Tracks market liquidity.

Fields include:

* Vault address
* LP mint
* Total liquidity
* Total LP shares
* Locked collateral

---

### Position

Represents a user's open perpetual position.

Fields include:

* Owner
* Market ID
* Position side
* Collateral
* Position size
* Leverage
* Entry price
* Realized PnL
* Open timestamp

---

### Treasury

Protocol fee collection account.

Stores accumulated protocol fees.

---

# Program Instructions

## Initialize

Initializes protocol state.

Creates:

* Global state
* Treasury

Only executed once.

---

## Create Market

Creates a new perpetual market.

Parameters:

* Market ID
* Oracle address
* Maximum leverage
* Maintenance margin
* Open fee
* Close fee
* Liquidation fee
* Oracle staleness limit
* Confidence threshold

Creates:

* Market account
* Liquidity pool
* LP token mint
* Market vault

---

## Add Liquidity

Deposits liquidity into a market pool.

Actions:

1. Transfer collateral to vault
2. Mint LP shares
3. Update liquidity accounting

---

## Remove Liquidity

Withdraws liquidity from a market pool.

Actions:

1. Burn LP shares
2. Return proportional liquidity
3. Update pool accounting

---

## Open Position

Opens a leveraged perpetual position.

Parameters:

* Market ID
* Side (Long/Short)
* Collateral
* Position Size
* Leverage

Validations:

* Minimum collateral
* Market active
* Oracle validity
* Leverage constraints
* Margin requirements

---

## Close Position

Closes an existing position.

Actions:

* Fetch oracle price
* Calculate PnL
* Apply trading fees
* Return remaining collateral
* Release locked liquidity

---

## Liquidate

Liquidates unhealthy positions.

Conditions:

* Margin below maintenance threshold

Actions:

* Close position
* Apply liquidation penalty
* Reward liquidator
* Release collateral

---

## Pause Market

Temporarily pauses or resumes trading.

Effects:

* Prevents new positions
* Existing positions remain manageable

---

## Update Market

Updates market risk parameters.

Configurable:

* Oracle
* Leverage limits
* Margin requirements
* Fees
* Oracle validation settings

---

# Mathematical Components

## Position Margin

Required margin:

```text
Required Margin = Position Size / Leverage
```

---

## Trading Fees

Open Fee:

```text
Open Fee = Collateral × Open Fee BPS
```

Close Fee:

```text
Close Fee = Position Value × Close Fee BPS
```

---

## Profit and Loss

Long Position:

```text
PnL = Position Size × (Current Price - Entry Price) / Entry Price
```

Short Position:

```text
PnL = Position Size × (Entry Price - Current Price) / Entry Price
```

---

## Liquidation Check

A position becomes liquidatable when:

```text
Margin Ratio < Maintenance Margin Requirement
```

---

# Project Structure

```text
programs/
└── perps/
    └── src/
        ├── constants.rs
        ├── error.rs
        ├── lib.rs
        ├── instructions/
        │   ├── initialize.rs
        │   ├── create_market.rs
        │   ├── add_liquidity.rs
        │   ├── remove_liquidity.rs
        │   ├── open_position.rs
        │   ├── close_position.rs
        │   ├── liquidate.rs
        │   ├── pause_market.rs
        │   └── update_market.rs
        ├── state/
        │   ├── global_state.rs
        │   ├── market.rs
        │   ├── liquidity_pool.rs
        │   ├── position.rs
        │   └── treasury.rs
        └── math/
            ├── pnl.rs
            ├── fees.rs
            └── liquidation.rs

client/
└── src/
    ├── perps.ts
    └── example/
        └── PerpsExample.tsx
```

---

# Installation

## Prerequisites

* Rust
* Solana CLI
* Anchor Framework
* Node.js
* Yarn or npm

---

## Clone Repository

```bash
git clone <repository-url>
cd perps-protocol
```

---

## Install Dependencies

```bash
npm install
```

or

```bash
yarn install
```

---

## Build Program

```bash
anchor build
```

---

## Run Local Validator

```bash
solana-test-validator
```

---

## Deploy

```bash
anchor deploy
```

---

# Client Usage

Generate TypeScript bindings:

```bash
anchor build
```

IDL output:

```text
client/src/perps_idl.json
```

Example integration:

```typescript
import { PerpsClient } from "./perps";
```

See:

```text
client/src/example/PerpsExample.tsx
```

for sample usage.

---

# Security Considerations

* Oracle freshness validation
* Oracle confidence validation
* Leverage restrictions
* Maintenance margin enforcement
* Overflow-safe arithmetic
* PDA-based account ownership
* Admin-controlled emergency pause

---

# Future Improvements

* Cross-margin accounts
* Funding rate mechanism
* Multiple collateral assets
* Dynamic fee model
* Advanced liquidation engine
* Insurance fund
* Orderbook integration
* TWAP oracle support
* Referral rewards
* Permissionless market creation

---

# License

MIT License

---

# Disclaimer

This software is experimental and provided for educational and research purposes.

Do not use with significant capital without extensive auditing, testing, and security review.
