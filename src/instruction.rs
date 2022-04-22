use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_program::{system_program, sysvar};

use crate::state::DepositHistoryData;
use crate::{id, ADMIN_PUBKEY};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum DepositInstructions {
    /// Deposit lamports to the deposit account
    ///
    /// Accounts expected:
    /// 0. `[signer, writable]` The account of the person who wants to send the donation
    /// 1. `[writable]` The deposit accumulate account
    /// 2. `[writable]` The PDA account for storing data
    /// 3. `[]` System program
    Deposit { amount: u64 },

    /// Send all deposited lamports to admin account
    ///
    /// Accounts expected:
    /// 0. `[signer, writable]` The admin account
    /// 1. `[writable]` The deposit accumulate account
    /// 2. `[]` Rent sysvar
    Withdraw,

    /// Create PDA and deposit accounts
    ///
    /// Accounts expected:
    /// 0. `[signer, writable]` The admin account
    /// 1. `[writable]` The PDA account for storing data
    /// 2. `[]` Rent sysvar
    /// 3. `[]` System program
    Initialize,
}

impl DepositInstructions {
    pub fn create_deposit(user: &Pubkey, amount: u64) -> Instruction {
        let (pda_pubkey, _) = DepositHistoryData::get_pda_pubkey_with_bump();
        let (deposit_pubkey, _) = DepositHistoryData::get_deposit_with_bump();
        Instruction::new_with_borsh(
            id(),
            &DepositInstructions::Deposit { amount },
            vec![
                AccountMeta::new(user.clone(), true),
                AccountMeta::new(deposit_pubkey, false),
                AccountMeta::new(pda_pubkey, false),
                AccountMeta::new(system_program::id(), false),
            ],
        )
    }

    pub fn create_withdraw() -> Instruction {
        let (deposit_pubkey, _) = DepositHistoryData::get_deposit_with_bump();
        Instruction::new_with_borsh(
            id(),
            &DepositInstructions::Withdraw,
            vec![
                AccountMeta::new(ADMIN_PUBKEY.clone(), true),
                AccountMeta::new(deposit_pubkey, false),
                AccountMeta::new_readonly(sysvar::rent::id(), false),
            ],
        )
    }

    pub fn create_initialize() -> Instruction {
        let (pda_pubkey, _) = DepositHistoryData::get_pda_pubkey_with_bump();
        let (deposit_pubkey, _) = DepositHistoryData::get_deposit_with_bump();
        Instruction::new_with_borsh(
            id(),
            &DepositInstructions::Initialize,
            vec![
                AccountMeta::new(ADMIN_PUBKEY.clone(), true),
                AccountMeta::new(pda_pubkey, false),
                AccountMeta::new(deposit_pubkey, false),
                AccountMeta::new_readonly(sysvar::rent::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        )
    }
}

#[cfg(test)]
mod test {
    use borsh::BorshSerialize;

    use crate::instruction::DepositInstructions;

    #[test]
    fn test_serialization_make_donation() {
        let data = DepositInstructions::Deposit { amount: 99 }
            .try_to_vec()
            .unwrap();
        assert_eq!(data, [0, 99, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_serialization_create_donation_account() {
        let data = DepositInstructions::Initialize.try_to_vec().unwrap();
        assert_eq!(data, [2]);
    }
}
