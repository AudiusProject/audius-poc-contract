//! Program state processor

use crate::instruction::TemplateInstruction;
use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    program::invoke, pubkey::Pubkey,
};

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// Call Audius program to verify signature
    pub fn process_example_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        signature_data: audius::instruction::SignatureData,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        // initialized valid signer account
        let valid_signer_info = next_account_info(account_info_iter)?;
        // signer group account
        let signer_group_info = next_account_info(account_info_iter)?;
        // audius account
        let audius_account_info = next_account_info(account_info_iter)?;
        // sysvar instruction
        let sysvar_instruction = next_account_info(account_info_iter)?;

        invoke(
            &audius::instruction::validate_signature_with_sysvar(
                &audius::id(),
                valid_signer_info.key,
                signer_group_info.key,
                sysvar_instruction.key,
                signature_data,
            )
            .unwrap(),
            &[
                audius_account_info.clone(),
                valid_signer_info.clone(),
                signer_group_info.clone(),
                sysvar_instruction.clone(),
            ],
        )?;

        Ok(())
    }

    /// Processes an instruction
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = TemplateInstruction::unpack(input)?;
        match instruction {
            TemplateInstruction::ExampleInstruction(signature_data) => {
                msg!("Instruction: ExampleInstruction");
                Self::process_example_instruction(program_id, accounts, signature_data)
            }
        }
    }
}
