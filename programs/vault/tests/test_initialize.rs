use anchor_lang::{InstructionData, ToAccountMetas};
use litesvm::LiteSVM;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_message::Message;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::Transaction;
use vault::{
    accounts::{Close, Deposit, Initialize, Withdraw},
    instruction::{
        Close as CloseIx, Deposit as DepositIx, Initialize as InitializeIx,
        Withdraw as WithdrawIx,
    },
    ID as PROGRAM_ID,
};

const SYSTEM_PROGRAM: Pubkey = solana_system_interface::program::ID;

fn setup() -> (LiteSVM, Keypair) {
    let mut svm = LiteSVM::new();
    let so_path = format!("{}/../../target/deploy/vault.so", env!("CARGO_MANIFEST_DIR"));
    svm.add_program_from_file(PROGRAM_ID, so_path).unwrap();
    let user = Keypair::new();
    svm.airdrop(&user.pubkey(), 10_000_000_000).unwrap();
    (svm, user)
}

fn pdas(user: &Pubkey) -> (Pubkey, Pubkey) {
    let (vault_state, _) = Pubkey::find_program_address(&[b"vault_state", user.as_ref()], &PROGRAM_ID);
    let (vault, _) = Pubkey::find_program_address(&[b"vault", user.as_ref()], &PROGRAM_ID);
    (vault_state, vault)
}

fn send(svm: &mut LiteSVM, payer: &Keypair, ix: Instruction) -> Result<(), String> {
    let message = Message::new(&[ix], Some(&payer.pubkey()));
    let tx = Transaction::new(&[payer], message, svm.latest_blockhash());
    svm.send_transaction(tx).map(|_| ()).map_err(|e| format!("{e:?}"))
}

#[test]
fn full_lifecycle_works() {
    let (mut svm, user) = setup();
    let (vault_state, vault) = pdas(&user.pubkey());

    // initialize
    let ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: Initialize { user: user.pubkey(), vault_state, vault, system_program: SYSTEM_PROGRAM }
            .to_account_metas(None),
        data: InitializeIx.data(),
    };
    send(&mut svm, &user, ix).unwrap();

    // deposit
    let ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: Deposit { user: user.pubkey(), vault, system_program: SYSTEM_PROGRAM }.to_account_metas(None),
        data: DepositIx { amount: 1_000_000 }.data(),
    };
    send(&mut svm, &user, ix).unwrap();
    let balance_after_deposit = svm.get_account(&vault).unwrap().lamports;
    assert!(balance_after_deposit >= 1_000_000);

    // withdraw
    let ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: Withdraw { user: user.pubkey(), vault, vault_state, system_program: SYSTEM_PROGRAM }
            .to_account_metas(None),
        data: WithdrawIx { amount: 500_000 }.data(),
    };
    send(&mut svm, &user, ix).unwrap();
    let balance_after_withdraw = svm.get_account(&vault).unwrap().lamports;
    assert_eq!(balance_after_withdraw, balance_after_deposit - 500_000);

    // close
    let ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: Close { user: user.pubkey(), vault, vault_state, system_program: SYSTEM_PROGRAM }
            .to_account_metas(None),
        data: CloseIx.data(),
    };
    send(&mut svm, &user, ix).unwrap();
    assert!(svm.get_account(&vault_state).is_none(), "vault_state should be closed");
}

#[test]
fn cannot_initialize_twice() {
    let (mut svm, user) = setup();
    let (vault_state, vault) = pdas(&user.pubkey());

    let ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: Initialize { user: user.pubkey(), vault_state, vault, system_program: SYSTEM_PROGRAM }
            .to_account_metas(None),
        data: InitializeIx.data(),
    };
    send(&mut svm, &user, ix.clone()).unwrap();
    assert!(send(&mut svm, &user, ix).is_err());
}

#[test]
fn cannot_withdraw_more_than_deposited() {
    let (mut svm, user) = setup();
    let (vault_state, vault) = pdas(&user.pubkey());

    let ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: Initialize { user: user.pubkey(), vault_state, vault, system_program: SYSTEM_PROGRAM }
            .to_account_metas(None),
        data: InitializeIx.data(),
    };
    send(&mut svm, &user, ix).unwrap();

    let ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: Deposit { user: user.pubkey(), vault, system_program: SYSTEM_PROGRAM }.to_account_metas(None),
        data: DepositIx { amount: 1_000_000 }.data(),
    };
    send(&mut svm, &user, ix).unwrap();

    let ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: Withdraw { user: user.pubkey(), vault, vault_state, system_program: SYSTEM_PROGRAM }
            .to_account_metas(None),
        data: WithdrawIx { amount: 5_000_000_000 }.data(),
    };
    assert!(send(&mut svm, &user, ix).is_err());
}