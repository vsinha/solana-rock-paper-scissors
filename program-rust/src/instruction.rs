use crate::rps::RockPaperScissorsMove;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum MyInstruction {
    Greeting {
        id: u32,
    },
    RPSSet {
        sender: Pubkey,
        rps_move: RockPaperScissorsMove,
    },
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

pub fn rps_send(
    program_id: &Pubkey,
    greeted_pubkey: &Pubkey,
    rps_move: RockPaperScissorsMove,
    sender: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let data = MyInstruction::RPSSet {
        sender: *sender,
        rps_move,
    };

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
