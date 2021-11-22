//! Program entrypoint

use crate::hello_world;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum MyInstruction {
    Greeting { id: u32 },
}

pub fn greeting(
    program_id: &Pubkey,
    greeted_pubkey: &Pubkey,
    id: u32,
) -> Result<Instruction, ProgramError> {
    let data = MyInstruction::Greeting { id };

    let mut accounts = Vec::with_capacity(1);
    accounts.push(AccountMeta::new(*greeted_pubkey, false));

    Ok(Instruction::new_with_borsh(*program_id, &data, accounts))
}

impl MyInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        println!("{}", input.len());

        solana_program::borsh::try_from_slice_unchecked::<MyInstruction>(input)
            .map_err(|_| ProgramError::InvalidArgument)
    }
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    println!("process input: {:?}", input);
    let instruction = MyInstruction::unpack(input)?;

    match instruction {
        MyInstruction::Greeting { id: _ } => hello_world::process(program_id, accounts),
    }
}

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = process(program_id, accounts, instruction_data) {
        // // catch the error so we can print it
        // error.print::<ProgramError>();
        return Err(error);
    }
    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;
    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let instruction_data: Vec<u8> = MyInstruction::Greeting { id: 0 }.try_to_vec().unwrap();

        let accounts = vec![account];

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
    }
}
