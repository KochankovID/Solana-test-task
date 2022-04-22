use solana_program::pubkey;
use solana_program::pubkey::Pubkey;

pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

pub const DEPOSIT_HISTORY_SEED: &str = "deposit-history-seed";
pub const DEPOSIT_SEED: &str = "deposit";
pub const ADMIN_PUBKEY: Pubkey = pubkey!("3N7dHiEv6fz59uwNBTMNp9Fei9JKWL6je1fUnDxWXdbQ");
solana_program::declare_id!("AkCLhVcBtdSs2erJ5X129pQaTE6dqzhP8ou6AtZUBQkQ");
