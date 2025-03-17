use anchor_client::{
    anchor_lang::{accounts::sysvar, system_program},
    solana_sdk::{
        self, clock,
        commitment_config::CommitmentConfig,
        instruction::Instruction,
        pubkey::Pubkey,
        rent::Rent,
        signature::{read_keypair_file, Keypair},
        signer::{SeedDerivable, Signer},
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
use defi_market;
use crate::constants::{
    DEFI_MARKETS_PROGRAM_ID, LENDING_MARKET, LENDING_MARKET_AUTHORITY, RESERVE,
    RESERVE_COLLATERAL_MINT, RESERVE_LIQUIDITY_SUPPLY, SOLEND_PROGRAM_ID,
};

#[test]
fn test_deposit_obligatory_collateral() -> Result<(), Box<dyn std::error::Error>> {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();
    /// clients
    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());
    //**************PROGRAMS***************/
    /// system
    let _system_program = client.program(system_program::ID).unwrap();
    let system_program_client = _system_program.rpc();
    /// Defimarkets
    let defi_markets_program_id = Pubkey::from_str(DEFI_MARKETS_PROGRAM_ID).unwrap();
    let defi_markets_program = client.program(defi_markets_program_id).unwrap();
   
    let solend_program_id = Pubkey::from_str(SOLEND_PROGRAM_ID).unwrap();
 
   let token_program_id = anchor_spl::token::ID;
    let payer_pubkey = payer.pubkey().clone();
    ///
    let lending_market = Pubkey::from_str(&LENDING_MARKET).unwrap();
    let mut seed: Vec<u8> = b"obligation4".to_vec();
    seed.extend_from_slice(&payer_pubkey.to_bytes());  
    let account_keypair = Keypair::from_seed(&seed).unwrap();
    let account_pubkey = account_keypair.pubkey();
    let space = 2040;
    let lamports = system_program_client.get_minimum_balance_for_rent_exemption(space)?; // Adjust size as needed
    
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &account_pubkey, // AQi4hXeyynVGHgR7spi1v6Bx1ePtJBH94VpqBCWKoiQW
        lamports,
        space as u64, // Account data size (adjust if needed)
        &solend_program_id, // Set ownership to the native program
    );

    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix],
        Some(&payer.pubkey()),
        &[&payer, &account_keypair], // Signers
        system_program_client.get_latest_blockhash()?,
    );
 

    let tx = defi_markets_program
        .request()
        .accounts(defi_market::accounts::InitObligation {
            payer: payer.pubkey(),
            obligation:  account_pubkey,
            lending_market: Pubkey::from_str(&LENDING_MARKET).unwrap(),
            obligation_owner: payer.pubkey(),
            rent: Rent::id(), 
            token_program_id,
            lending_program_id: Pubkey::from_str(SOLEND_PROGRAM_ID).unwrap(),
            system_program: system_program::ID,
        })
        .args(defi_market::instruction::InitObligation {})
        .signer(&payer)
        .send()
        .expect("");

 
    Ok(())
}
 