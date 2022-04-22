# Solana-test-task

---

## Setup

### Solana cli

- Run `solana config set --url localhost`
- Run `solana-test-validator`

## Run

- Run `cargo build`

## Description
### Overview

The program creates two accounts:
- Deposit account for storing sol tokens (data is empty)
- PDA account for storing information about user's deposit

### Instructions
- Deposit { amount: u64 } - Deposit lamports to the deposit account
  - `[signer, writable]` - The account of the person who wants to send the donation
  - `[writable]` - The deposit accumulate account
  - `[writable]` The PDA account for storing data
  - `[]` System program
- Withdraw - Send all deposited lamports to admin account
  - `[signer, writable]` Admin account
  - `[writable]` The deposit accumulate account
  - `[]` Rent sysvar
- Initialize - Create PDA and deposit accounts
  - `[signer, writable]` The admin account
  - `[writable]` The PDA account for storing data
  - `[]` Rent sysvar
  - `[]` System program