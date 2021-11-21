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
    let data = MyInstruction::Greeting { id }.try_to_vec()?;

    println!("{:?}", data);

    let mut accounts = Vec::with_capacity(1);
    accounts.push(AccountMeta::new(*greeted_pubkey, false));

    Ok(Instruction::new_with_borsh(
        *program_id,
        &data.as_slice(),
        accounts,
    ))
}

impl MyInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use ProgramError::InvalidArgument;

        MyInstruction::try_from_slice(input).map_err(|_| InvalidArgument)
    }
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    println!("{:?}", input);
    let instruction = MyInstruction::unpack(input)?;

    match instruction {
        MyInstruction::Greeting { id: _ } => {
            // msg!("Instruction: InitializeMint");
            hello_world::process(program_id, accounts)
        }
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
