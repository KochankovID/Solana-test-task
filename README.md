# Solana-test-task

---

## Setup

- Run `cargo install`
- Run `cd client && npm i`

## Run

- Run `cargo build`

## Run unit tests

- Run `cargo test`

## Run integration tests

- Run `cargo test-bpf`

## Run client

- Run `cd client && npm run run`

## Description

### Overview

The program creates two accounts:

- Deposit account for storing sol tokens (data is empty)
- PDA account for storing information about user's deposit

### Instructions

- Deposit { amount: u64 } - Deposit lamports to the deposit account
  - `[signer, writable]` - The account of the person who wants to send the donation
  - `[writable]` - The deposit accumulate account
  - `[writable]` The PDA account for storing history data
  - `[]` System program
- Withdraw - Send all deposited lamports to admin account
  - `[signer, writable]` Admin account
  - `[writable]` The deposit accumulate account
  - `[]` Rent sysvar
- Initialize - Create PDA and deposit accounts
  - `[signer, writable]` The admin account
  - `[writable]` The PDA account for storing history data
  - `[writable]` The deposit accumulate account
  - `[]` Rent sysvar
  - `[]` System program

## Addresses

- program: 3jYkeV2vknPL5UgFxANiNBUnRJuGeZcBP22C2gZJ1BT7
- admin: 3N7dHiEv6fz59uwNBTMNp9Fei9JKWL6je1fUnDxWXdbQ
