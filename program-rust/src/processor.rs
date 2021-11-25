use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::{hello_world, instruction::MyInstruction, rps::process_rps_set};

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    println!("process input: {:?}", input);
    let instruction = MyInstruction::unpack(input)?;

    match instruction {
        MyInstruction::Greeting { id: _ } => hello_world::process(program_id, accounts),
        MyInstruction::RPSSet { rps_move, sender } => {
            process_rps_set(program_id, accounts, rps_move, sender)
        }
    }
}

// Sanity tests
#[cfg(test)]
mod test {
    use crate::instruction::MyInstruction;

    use super::*;
    use borsh::BorshSerialize;
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

        process(&program_id, &accounts, &instruction_data).unwrap();
    }
}
