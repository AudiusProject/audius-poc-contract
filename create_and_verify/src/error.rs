//! Error types

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

/// Errors that may be returned by the CreateAndVerify program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum ProgramTemplateError {
    /// Example error
    #[error("Example error")]
    ExampleError,
    /// Instruction unpack error
    #[error("Instruction unpack error")]
    InstructionUnpackError,
    /// Invalid track data were passed
    #[error("Invalid track data were passed")]
    InvalidTrackData,
    /// Invalid verifier
    #[error("Invalid verifier account")]
    InvalidVerifierAccount
}
impl From<ProgramTemplateError> for ProgramError {
    fn from(e: ProgramTemplateError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for ProgramTemplateError {
    fn type_of() -> &'static str {
        "ProgramTemplateError"
    }
}

impl PrintProgramError for ProgramTemplateError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            ProgramTemplateError::ExampleError => msg!("Example error message"),
            ProgramTemplateError::InstructionUnpackError => msg!("Instruction unpack error"),
            ProgramTemplateError::InvalidTrackData => msg!("Invalid track data were passed"),
            ProgramTemplateError::InvalidVerifierAccount => msg!("Invalid verifier account provided"),
        }
    }
}
