# CONTRACT ID

# CONTRACT LINK:https://stellar.expert/explorer/testnet/contract/CCJXGOGOAOLTHQLU5MJMBSJJRJXOQCEAJ32WNNDSZKPWOS34HABPUCNL


![PICTURE](STELLAR.PNG)



# AgroSplit

Instant, transparent harvest payment splits for smallholder farmers.

## Problem

Smallholder farmers in the Philippines wait 45–60 days for cooperative payments, with intermediaries taking cuts at each step. This delays their ability to purchase inputs for the next season.

## Solution

A Soroban smart contract that automatically splits harvest payments the moment produce is weighed: 85% to the farmer, 10% to cooperative operations, and 5% to a community savings pool—all in under 5 seconds.

## Timeline

- **Week 1:** Contract development and testing
- **Week 2:** Basic mobile UI with QR scanning
- **Week 3:** Integration with testnet and demo preparation

## Stellar Features Used

- USDC transfers for stable value
- Soroban smart contracts for programmable splits
- Custom tokens for cooperative share tracking

## Vision and Purpose

Eliminate payment delays and intermediary fees for 2 million Filipino smallholder farmers, unlocking working capital and reducing reliance on predatory lenders.

## Prerequisites

- Rust 1.74+
- Soroban CLI v21.0.0+
- Stellar testnet account with USDC

## Build

```bash
soroban contract build
