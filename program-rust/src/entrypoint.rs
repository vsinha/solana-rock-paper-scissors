use crate::processor;
use solana_program::account_info::AccountInfo;
use solana_program::{entrypoint, entrypoint::ProgramResult, pubkey::Pubkey};

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = processor::process(program_id, accounts, instruction_data) {
        return Err(error);
    }
    Ok(())
}
