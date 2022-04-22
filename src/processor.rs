use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::native_token::sol_to_lamports;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_program::{msg, system_instruction};

use crate::error::DonationError;
use crate::instruction::DepositInstructions;
use crate::state::DepositHistoryData;
use crate::{id, ADMIN_PUBKEY, DEPOSIT_HISTORY_SEED, DEPOSIT_SEED};

pub struct Processor;

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = DepositInstructions::try_from_slice(instruction_data)?;
        msg!("Deposit: {:?}", instruction);

        match instruction {
            DepositInstructions::Deposit { amount } => Self::process_deposit(accounts, amount),
            DepositInstructions::Withdraw => Self::process_withdraw(accounts),
            DepositInstructions::Initialize => Self::process_initialize(accounts),
        }
    }

    fn process_deposit(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
        msg!("process deposit {}", amount);

        let acc_iter = &mut accounts.iter();

        let user_acc = next_account_info(acc_iter)?;
        let deposit_acc = next_account_info(acc_iter)?;
        let pda_acc = next_account_info(acc_iter)?;
        let system_acc = next_account_info(acc_iter)?;

        // Checks
        if !user_acc.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        msg!("User is correct");

        let (deposit_pubkey, _) = DepositHistoryData::get_deposit_with_bump();
        if *deposit_acc.key != deposit_pubkey {
            return Err(ProgramError::InvalidArgument);
        }

        msg!("Deposit account is correct");

        let (donation_pubkey, _) = DepositHistoryData::get_pda_pubkey_with_bump();
        if donation_pubkey != *pda_acc.key {
            return Err(ProgramError::InvalidArgument);
        }

        msg!("PDA account is initialized");

        invoke(
            &system_instruction::transfer(user_acc.key, deposit_acc.key, amount),
            &[user_acc.clone(), deposit_acc.clone(), system_acc.clone()],
        )?;

        msg!(
            "transfer {} lamports from {:?} to {:?}: done",
            amount,
            user_acc.key,
            deposit_acc.key
        );

        let mut deposit_history_data: DepositHistoryData =
            DepositHistoryData::deserialize(&mut &pda_acc.data.borrow()[..])?;

        msg!("DepositHistoryData is deserialized");

        let saved_amount = deposit_history_data
            .history
            .get(&user_acc.key.to_bytes())
            .unwrap_or(&0)
            .clone();
        deposit_history_data
            .history
            .insert(user_acc.key.to_bytes(), amount + saved_amount);
        deposit_history_data.serialize(&mut &mut pda_acc.data.borrow_mut()[..])?;

        msg!("DepositHistoryData is serialized");

        Ok(())
    }

    fn process_withdraw(accounts: &[AccountInfo]) -> ProgramResult {
        msg!("process withdraw");

        let acc_iter = &mut accounts.iter();

        let admin_acc = next_account_info(acc_iter)?;
        let deposit_acc = next_account_info(acc_iter)?;
        let rent_acc = next_account_info(acc_iter)?;

        // Checks
        if !admin_acc.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if *admin_acc.key != ADMIN_PUBKEY {
            return Err(DonationError::AdminRequired.into());
        }

        msg!("Admin is correct");

        let (deposit_pubkey, _) = DepositHistoryData::get_deposit_with_bump();
        if *deposit_acc.key != deposit_pubkey {
            return Err(ProgramError::InvalidArgument);
        }

        msg!("Deposit account is correct");

        let amount = **deposit_acc.lamports.borrow();
        if amount < sol_to_lamports(0.01) {
            return Err(ProgramError::InsufficientFunds);
        }
        let rent = &Rent::from_account_info(rent_acc)?;
        let amount = amount - rent.minimum_balance(0);

        **deposit_acc.try_borrow_mut_lamports()? -= amount;
        **admin_acc.try_borrow_mut_lamports()? += amount;

        msg!(
            "withdraw {} lamports from {:?} to {:?}: done",
            amount,
            deposit_acc.key,
            admin_acc.key
        );

        Ok(())
    }

    fn process_initialize(accounts: &[AccountInfo]) -> ProgramResult {
        msg!("process process_creation_pda");

        let acc_iter = &mut accounts.iter();

        let admin_acc = next_account_info(acc_iter)?;
        let pda_acc = next_account_info(acc_iter)?;
        let deposit_acc = next_account_info(acc_iter)?;
        let rent_acc = next_account_info(acc_iter)?;
        let system_program_acc = next_account_info(acc_iter)?;

        // Checks
        if !admin_acc.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if *admin_acc.key != ADMIN_PUBKEY {
            return Err(DonationError::AdminRequired.into());
        }

        msg!("Admin is correct");

        let (pda_pubkey, pda_bump) = DepositHistoryData::get_pda_pubkey_with_bump();
        if *pda_acc.key != pda_pubkey {
            return Err(ProgramError::InvalidArgument);
        }

        if !pda_acc.data_is_empty() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        msg!("PDA is correct");

        let (deposit_pubkey, deposit_bump) = DepositHistoryData::get_deposit_with_bump();
        if *deposit_acc.key != deposit_pubkey {
            return Err(ProgramError::InvalidArgument);
        }

        msg!("Deposit is correct");

        let space = 60 * 100;
        let rent = &Rent::from_account_info(rent_acc)?;
        let lamports = rent.minimum_balance(space);
        let signer_seeds: &[&[_]] = &[DEPOSIT_HISTORY_SEED.as_bytes(), &[pda_bump]];
        invoke_signed(
            &system_instruction::create_account(
                admin_acc.key,
                &pda_pubkey,
                lamports,
                space as u64,
                &id(),
            ),
            &[
                admin_acc.clone(),
                pda_acc.clone(),
                system_program_acc.clone(),
            ],
            &[&signer_seeds],
        )?;

        msg!("Created PDA account");

        let signer_seeds: &[&[_]] = &[DEPOSIT_SEED.as_bytes(), &[deposit_bump]];
        let lamports = rent.minimum_balance(0);
        invoke_signed(
            &system_instruction::create_account(
                admin_acc.key,
                &deposit_pubkey,
                lamports,
                0u64,
                &id(),
            ),
            &[
                admin_acc.clone(),
                deposit_acc.clone(),
                system_program_acc.clone(),
            ],
            &[&signer_seeds],
        )?;

        msg!("Created deposit account");

        Ok(())
    }
}
