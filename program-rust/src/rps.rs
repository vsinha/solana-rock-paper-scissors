use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum RockPaperScissorsMove {
    Rock,
    Paper,
    Scissors,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum RockPaperScissorsOutcome {
    Winner { account: Pubkey },
    Draw,
}

#[derive(PartialEq, BorshSerialize, BorshDeserialize, Debug)]
pub enum RPSState {
    Nil,
    OneMoveStored {
        stored_move: RockPaperScissorsMove,
        stored_account: Pubkey,
    },
    Outcome {
        outcome: RockPaperScissorsOutcome,
    },
}

impl Default for RPSState {
    fn default() -> Self {
        RPSState::Nil
    }
}

impl RPSState {
    pub fn default_as_vec() -> Vec<u8> {
        let mut data = RPSState::default().try_to_vec().unwrap();
        data.resize(34, 0);
        data
    }
}

pub fn process_rps_set(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    sent_move: RockPaperScissorsMove,
    sender: Pubkey,
) -> ProgramResult {
    // Get the account which represents the game state
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut state = try_from_slice_unchecked::<RPSState>(&accounts[0].data.borrow())?;
    match state {
        RPSState::Nil => {
            msg!("Storing first move");
            state = RPSState::OneMoveStored {
                stored_move: sent_move,
                stored_account: sender,
            }
        }
        RPSState::OneMoveStored {
            stored_move,
            stored_account,
        } => {
            if stored_account == sender {
                msg!("You've played already, wait until the other player plays");
            } else {
                use RockPaperScissorsMove::*;
                use RockPaperScissorsOutcome::*;
                let outcome = match (stored_move, sent_move) {
                    (Paper, Paper) | (Rock, Rock) | (Scissors, Scissors) => Draw,
                    (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => {
                        Winner { account: sender }
                    }
                    _ => Winner {
                        account: stored_account,
                    },
                };
                state = RPSState::Outcome { outcome }
            }
        }
        RPSState::Outcome { outcome: _ } => {
            msg!("This round has already been played. Pass reset to start a new one");
        }
    };

    state.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::{borsh::try_from_slice_unchecked, clock::Epoch};

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let player1 = Pubkey::new_unique();
        let player2 = Pubkey::new_unique();
        let mut lamports = 0;
        let mut data = RPSState::default_as_vec();
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            data.as_mut_slice(),
            &owner,
            false,
            Epoch::default(),
        );
        let _instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        // check initial state
        assert_eq!(
            try_from_slice_unchecked::<RPSState>(&accounts[0].data.borrow()).unwrap(),
            RPSState::Nil
        );

        // set as p1
        process_rps_set(
            &program_id,
            &accounts,
            RockPaperScissorsMove::Paper,
            player1,
        )
        .unwrap();
        assert_eq!(
            try_from_slice_unchecked::<RPSState>(&accounts[0].data.borrow()).unwrap(),
            RPSState::OneMoveStored {
                stored_move: RockPaperScissorsMove::Paper,
                stored_account: player1
            }
        );

        // try to set as p1 again (expect no change)
        process_rps_set(
            &program_id,
            &accounts,
            RockPaperScissorsMove::Paper,
            player1,
        )
        .unwrap();

        assert_eq!(
            try_from_slice_unchecked::<RPSState>(&accounts[0].data.borrow()).unwrap(),
            RPSState::OneMoveStored {
                stored_move: RockPaperScissorsMove::Paper,
                stored_account: player1
            }
        );

        // try to set as p1 again (expect no change)
        process_rps_set(&program_id, &accounts, RockPaperScissorsMove::Rock, player2).unwrap();

        assert_eq!(
            try_from_slice_unchecked::<RPSState>(&accounts[0].data.borrow()).unwrap(),
            RPSState::Outcome {
                outcome: RockPaperScissorsOutcome::Winner { account: player1 }
            }
        );
    }
}
