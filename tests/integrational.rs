#![cfg(feature = "test-bpf")]

use std::collections::HashMap;

use borsh::BorshSerialize;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::native_token::sol_to_lamports;
use solana_program::{system_instruction, sysvar};
use solana_program_test::{processor, tokio, ProgramTest, ProgramTestContext};
use solana_sdk::account::{Account, WritableAccount};
use solana_sdk::pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use solana_test_task::entrypoint::process_instruction;
use solana_test_task::id;
use solana_test_task::instruction::DepositInstructions;
use solana_test_task::state::DepositHistoryData;

struct Env {
    ctx: ProgramTestContext,
    admin: Keypair,
    user: Keypair,
}

impl Env {
    async fn new() -> Self {
        let program_test =
            ProgramTest::new("solana_test_task", id(), processor!(process_instruction));
        let mut ctx = program_test.start_with_context().await;

        let admin = Keypair::from_bytes(&[
            203, 219, 86, 187, 107, 81, 112, 226, 4, 227, 158, 252, 76, 123, 149, 180, 95, 198, 36,
            9, 235, 156, 55, 45, 74, 84, 77, 104, 33, 95, 92, 16, 35, 32, 15, 255, 219, 159, 176,
            79, 195, 212, 154, 21, 69, 187, 78, 252, 114, 21, 13, 226, 204, 217, 246, 16, 100, 38,
            1, 39, 21, 32, 244, 59,
        ])
        .unwrap();
        let user = Keypair::new();

        // credit admin and user accounts
        ctx.banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[
                    system_instruction::transfer(
                        &ctx.payer.pubkey(),
                        &admin.pubkey(),
                        5_000_000_000,
                    ),
                    system_instruction::transfer(
                        &ctx.payer.pubkey(),
                        &user.pubkey(),
                        5_000_000_000,
                    ),
                ],
                Some(&ctx.payer.pubkey()),
                &[&ctx.payer],
                ctx.last_blockhash,
            ))
            .await
            .unwrap();

        // init donation account
        let tx = Transaction::new_signed_with_payer(
            &[DepositInstructions::create_initialize()],
            Some(&admin.pubkey()),
            &[&admin],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        Env { ctx, admin, user }
    }
}

#[tokio::test]
async fn test_setup() {
    let mut env: Env = Env::new().await;
    let (pda_pubkey, _) = DepositHistoryData::get_pda_pubkey_with_bump();
    let (deposit_pubkey, _) = DepositHistoryData::get_deposit_with_bump();

    assert_eq!(
        env.ctx
            .banks_client
            .get_account(pubkey!(pda_pubkey))
            .await
            .unwrap()
            .unwrap(),
        Account::new(42650880, 60 * 100, &id(),)
    );

    assert_eq!(
        env.ctx
            .banks_client
            .get_account(pubkey!(deposit_pubkey))
            .await
            .unwrap()
            .unwrap(),
        Account::new(890880, 0, &id(),)
    );
}

#[tokio::test]
async fn test_make_donation() {
    let mut env: Env = Env::new().await;
    let (pda_pubkey, _) = DepositHistoryData::get_pda_pubkey_with_bump();
    let (deposit_pubkey, _) = DepositHistoryData::get_deposit_with_bump();

    let tx = Transaction::new_signed_with_payer(
        &[DepositInstructions::create_deposit(
            &env.user.pubkey(),
            sol_to_lamports(0.01),
        )],
        Some(&env.user.pubkey()),
        &[&env.user],
        env.ctx.last_blockhash,
    );
    env.ctx.banks_client.process_transaction(tx).await.unwrap();

    assert_eq!(
        env.ctx
            .banks_client
            .get_account(deposit_pubkey)
            .await
            .unwrap()
            .unwrap(),
        Account::new(
            10890880,
            0,
            &pubkey!("AkCLhVcBtdSs2erJ5X129pQaTE6dqzhP8ou6AtZUBQkQ"),
        )
    );

    let mut history = HashMap::new();
    history.insert(env.user.pubkey().to_bytes(), sol_to_lamports(0.01));

    let deposit_history_data = DepositHistoryData { history };
    let mut data = deposit_history_data.try_to_vec().unwrap();
    data.resize(6000, 0);

    assert_eq!(
        env.ctx
            .banks_client
            .get_account(pubkey!(pda_pubkey))
            .await
            .unwrap()
            .unwrap(),
        Account::create(42650880, data, id(), false, 0,)
    );
}

#[tokio::test]
async fn test_withdraw() {
    let mut env: Env = Env::new().await;

    let tx = Transaction::new_signed_with_payer(
        &[DepositInstructions::create_deposit(
            &env.user.pubkey(),
            sol_to_lamports(2f64),
        )],
        Some(&env.user.pubkey()),
        &[&env.user],
        env.ctx.last_blockhash,
    );
    env.ctx.banks_client.process_transaction(tx).await.unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[DepositInstructions::create_withdraw()],
        Some(&env.admin.pubkey()),
        &[&env.admin],
        env.ctx.last_blockhash,
    );
    env.ctx.banks_client.process_transaction(tx).await.unwrap();

    assert_eq!(
        env.ctx
            .banks_client
            .get_account(env.admin.pubkey())
            .await
            .unwrap()
            .unwrap(),
        Account::new(6956448240, 0, &pubkey!("11111111111111111111111111111111"),)
    );
}

#[tokio::test]
#[should_panic(
    expected = "called `Result::unwrap()` on an `Err` value: TransactionError(InstructionError(0, Custom(0)))"
)]
async fn test_user_cant_withdraw() {
    let mut env: Env = Env::new().await;

    let tx = Transaction::new_signed_with_payer(
        &[DepositInstructions::create_deposit(
            &env.user.pubkey(),
            sol_to_lamports(2f64),
        )],
        Some(&env.user.pubkey()),
        &[&env.user],
        env.ctx.last_blockhash,
    );
    env.ctx.banks_client.process_transaction(tx).await.unwrap();

    let (deposit_pubkey, _) = DepositHistoryData::get_deposit_with_bump();
    let tx = Transaction::new_signed_with_payer(
        &[Instruction::new_with_borsh(
            id(),
            &DepositInstructions::Withdraw,
            vec![
                AccountMeta::new(env.user.pubkey(), true),
                AccountMeta::new(deposit_pubkey, false),
                AccountMeta::new_readonly(sysvar::rent::id(), false),
            ],
        )],
        Some(&env.user.pubkey()),
        &[&env.user],
        env.ctx.last_blockhash,
    );
    env.ctx.banks_client.process_transaction(tx).await.unwrap();
}
