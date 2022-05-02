use std::collections::HashMap;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::{id, DEPOSIT_HISTORY_SEED, DEPOSIT_SEED};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub struct DepositHistoryData {
    pub history: HashMap<[u8; 32], u64>,
}

impl DepositHistoryData {
    pub fn get_pda_pubkey_with_bump() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[DEPOSIT_HISTORY_SEED.as_bytes()], &id())
    }

    pub fn get_deposit_with_bump() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[DEPOSIT_SEED.as_bytes()], &id())
    }
}

#[cfg(test)]
mod test {
    use borsh::BorshSerialize;

    use crate::pubkey;
    use crate::state::*;

    #[test]
    fn test_serialization() {
        let mut data = DepositHistoryData {
            history: HashMap::new(),
        };
        data.history.insert(
            pubkey!("GizgqMPamZ5joAZ8XxLPqshwvqD8xDFCp1buwhbi28sp").to_bytes(),
            100,
        );

        let serialized_data = data.try_to_vec().unwrap();
        assert_eq!(
            serialized_data,
            [
                1, 0, 0, 0, 233, 161, 87, 254, 99, 44, 51, 57, 46, 43, 81, 249, 106, 108, 21, 162,
                103, 253, 138, 211, 216, 110, 229, 99, 108, 33, 51, 118, 232, 151, 89, 119, 100, 0,
                0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn test_deserialization() {
        let serialized_data = [
            1, 0, 0, 0, 233, 161, 87, 254, 99, 44, 51, 57, 46, 43, 81, 249, 106, 108, 21, 162, 103,
            253, 138, 211, 216, 110, 229, 99, 108, 33, 51, 118, 232, 151, 89, 119, 100, 0, 0, 0, 0,
            0, 0, 0,
        ];
        let data = DepositHistoryData::try_from_slice(&serialized_data).unwrap();

        let mut expected_data = DepositHistoryData {
            history: HashMap::new(),
        };
        expected_data.history.insert(
            pubkey!("GizgqMPamZ5joAZ8XxLPqshwvqD8xDFCp1buwhbi28sp").to_bytes(),
            100,
        );

        assert_eq!(data, expected_data);
    }

    #[test]
    fn test_get_pda_address_with_seed() {
        let (address, bump) = DepositHistoryData::get_pda_pubkey_with_bump();
        assert_eq!(
            address,
            pubkey!("7jYpqqFSVDCGwTigh8a2vkcUfZKHGsrSVXAsfj8GeS7j")
        );
        assert_eq!(bump, 255);
    }

    #[test]
    fn test_get_deposit_address_with_seed() {
        let (address, bump) = DepositHistoryData::get_deposit_with_bump();
        assert_eq!(
            address,
            pubkey!("9Ry9NaGh9kKrBSxyUMWTsgRDVDfocUrDvMoZCbvh9LxC")
        );
        assert_eq!(bump, 255);
    }
}
