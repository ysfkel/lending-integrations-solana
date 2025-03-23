use anchor_client::{
    anchor_lang::{accounts::sysvar, system_program},
    solana_sdk::{
        clock,
        commitment_config::CommitmentConfig,
        instruction::Instruction,
        pubkey::Pubkey,
        signature::read_keypair_file,
        signer::Signer,
        system_instruction,
        sysvar::SysvarId,
        transaction::{self, Transaction},
    },
    Client, Cluster,
};
use anchor_spl::{
    associated_token::{
        get_associated_token_address,
        spl_associated_token_account::{self, instruction::create_associated_token_account},
        AssociatedToken,
    },
    token::spl_token::{instruction::sync_native, native_mint},
    token_2022::spl_token_2022::instruction::transfer_checked,
    token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use std::str::FromStr;

use crate::constants::{
    DEFI_MARKETS_PROGRAM_ID, LENDING_MARKET, LENDING_MARKET_AUTHORITY, RESERVE,
    RESERVE_COLLATERAL_MINT, RESERVE_LIQUIDITY_SUPPLY, SOLEND_PROGRAM_ID,
};

#[test]
fn test_deposit_reserve_liquidity() -> Result<(), Box<dyn std::error::Error>> {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();
 
    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(DEFI_MARKETS_PROGRAM_ID).unwrap();
    let program = client.program(program_id).unwrap();

    let token_program_id = anchor_spl::token::ID;
    let associated_token_program_id = anchor_spl::associated_token::ID;

    let collateral_mint = Pubkey::from_str(RESERVE_COLLATERAL_MINT).unwrap();
    // destination_collateral_account localnet -> BdgLzcRH36dJdm6Cz6YUeeUDoJzpEHjvVHATWJYJqo7j
    // destination_collateral_account devnet-> 3DkvmvL6ZwryiwtWaD1XF5QGgzBNxB2ZzpiciCaQmTxF 
    let destination_collateral_account = get_associated_token_address(
        &payer.pubkey(), // Owner
        &collateral_mint,
    );

    // Create the Associated Token Account instruction
    let create_associated_account_ix = create_associated_token_account(
        &payer.pubkey(),
        &payer.pubkey(),
        &collateral_mint,
        &token_program_id,
    );
 
    let source_liquidity_wsol_ata =
        get_associated_token_address(&payer.pubkey(), &native_mint::id());

    let create_associated_account_wrapped_sol_ix: Instruction = create_associated_token_account(
        &payer.pubkey(),
        &payer.pubkey(),
        &native_mint::id(),
        &token_program_id,
    );

    // transfer sol to wrapped account
    let transfer_sol_ix = system_instruction::transfer(
        &payer.pubkey(),
        &source_liquidity_wsol_ata,
        100_000_000_000,
    );

    let sync_native_ix = sync_native(&token_program_id, &source_liquidity_wsol_ata).unwrap();
    let sync_native = sync_native(&token_program_id, &source_liquidity_wsol_ata);
    let rpc_client: anchor_client::solana_client::rpc_client::RpcClient =
        client.program(associated_token_program_id).unwrap().rpc();
    let recent_blockhash = program.rpc().get_latest_blockhash()?;

    let create_ata_transaction = Transaction::new_signed_with_payer(
        &[
            create_associated_account_ix,
            create_associated_account_wrapped_sol_ix,
            transfer_sol_ix,
            sync_native_ix,
        ],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
   
 
   let signature = rpc_client.send_and_confirm_transaction(&create_ata_transaction)?;

    let amount: u64 = 100_000_000_000;  
    let tx = program
        .request()
        .accounts(defi_market::accounts::DepositReserveLiquidity {
            payer: payer.pubkey(),
            source_liquidity: source_liquidity_wsol_ata,
            destination_collateral: destination_collateral_account,
            reserve: Pubkey::from_str(RESERVE).unwrap(),
            reserve_liquidity_supply: Pubkey::from_str(RESERVE_LIQUIDITY_SUPPLY).unwrap(),
            reserve_collateral_mint: collateral_mint,
            lending_market: Pubkey::from_str(LENDING_MARKET).unwrap(),
            lending_market_authority: Pubkey::from_str(LENDING_MARKET_AUTHORITY).unwrap(),
            user_transfer_authority: payer.pubkey(),
            token_program_id,
            lending_program_id: Pubkey::from_str(SOLEND_PROGRAM_ID).unwrap(),
        })
        .args(defi_market::instruction::DepositReserveLiquidity { amount })
        .signer(&payer)
        .send()
        .expect("");

    let final_balance = rpc_client
        .get_token_account_balance(&source_liquidity_wsol_ata)?
        .amount
        .parse::<u64>()
        .unwrap_or(0);

    assert!(final_balance > 0);
 
    Ok(())
}
