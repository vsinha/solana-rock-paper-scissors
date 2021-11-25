use borsh::BorshDeserialize;
use rock_paper_scissors::entrypoint::process_instruction;
use rock_paper_scissors::hello_world::GreetingAccount;
use rock_paper_scissors::instruction::{greeting, rps_send};
use rock_paper_scissors::rps::{RPSState, RockPaperScissorsMove, RockPaperScissorsOutcome};
use solana_program::borsh::try_from_slice_unchecked;
use solana_program_test::*;
use solana_sdk::signature::Keypair;
use solana_sdk::{account::Account, pubkey::Pubkey, signature::Signer, transaction::Transaction};
use std::mem;

#[tokio::test]
async fn test_helloworld() {
    let program_id = Pubkey::new_unique();
    let greeted_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "helloworld", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );
    program_test.add_account(
        greeted_pubkey,
        Account {
            lamports: 5,
            data: vec![0_u8; mem::size_of::<u32>()],
            owner: program_id,
            ..Account::default()
        },
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Verify account has zero greetings
    let greeted_account = banks_client
        .get_account(greeted_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    assert_eq!(
        GreetingAccount::try_from_slice(&greeted_account.data)
            .unwrap()
            .counter,
        0
    );

    // Greet once
    let mut transaction = Transaction::new_with_payer(
        &[greeting(&program_id, &greeted_pubkey, 0).unwrap()],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer], recent_blockhash);
    let res = banks_client.process_transaction(transaction).await;
    println!("{:?}", res);
    res.unwrap();

    // Verify account has one greeting
    let greeted_account = banks_client
        .get_account(greeted_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    assert_eq!(
        GreetingAccount::try_from_slice(&greeted_account.data)
            .unwrap()
            .counter,
        1
    );

    // Greet again
    let mut transaction = Transaction::new_with_payer(
        &[greeting(&program_id, &greeted_pubkey, 1).unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account has two greetings
    let greeted_account = banks_client
        .get_account(greeted_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    assert_eq!(
        GreetingAccount::try_from_slice(&greeted_account.data)
            .unwrap()
            .counter,
        2
    );
}

#[tokio::test]
async fn test_rps() {
    let program_id = Pubkey::new_unique();
    let greeted_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "rps", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );
    program_test.add_account(
        greeted_pubkey,
        Account {
            lamports: 5,
            data: RPSState::default_as_vec(),
            owner: program_id,
            ..Account::default()
        },
    );
    let alice = Keypair::new();
    let bob = Keypair::new();
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let greeted_account = banks_client
        .get_account(greeted_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    assert_eq!(
        try_from_slice_unchecked::<RPSState>(&greeted_account.data).unwrap(),
        RPSState::Nil
    );

    let mut transaction = Transaction::new_with_payer(
        &[rps_send(
            &program_id,
            &greeted_pubkey,
            RockPaperScissorsMove::Paper,
            &alice.pubkey(),
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer], recent_blockhash);
    let res = banks_client.process_transaction(transaction).await;
    println!("{:?}", res);
    res.unwrap();

    // Verify account has one greeting
    let greeted_account = banks_client
        .get_account(greeted_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    assert_eq!(
        try_from_slice_unchecked::<RPSState>(&greeted_account.data).unwrap(),
        RPSState::OneMoveStored {
            stored_move: RockPaperScissorsMove::Paper,
            stored_account: alice.pubkey(),
        }
    );

    let mut transaction = Transaction::new_with_payer(
        &[rps_send(
            &program_id,
            &greeted_pubkey,
            RockPaperScissorsMove::Scissors,
            &bob.pubkey(),
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account has two greetings
    let greeted_account = banks_client
        .get_account(greeted_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    assert_eq!(
        try_from_slice_unchecked::<RPSState>(&greeted_account.data).unwrap(),
        RPSState::Outcome {
            outcome: RockPaperScissorsOutcome::Winner {
                account: bob.pubkey()
            }
        }
    );
}
