use solana_program::decode_error::DecodeError;
use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum DonationError {
    #[error("Admin signature is required")]
    AdminRequired,
}

impl From<DonationError> for ProgramError {
    fn from(e: DonationError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for DonationError {
    fn type_of() -> &'static str {
        "DonationError"
    }
}
