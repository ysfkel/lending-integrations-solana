use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;

use crate::instructions::constants::DEPOSIT_RESERVE_LIQUIDITY;

pub fn deposit_reserve_liquidity(ctx: Context<DepositReserveLiquidity>, amount: u64) -> Result<()> {
    let lending_program_id = ctx.accounts.lending_program_id.key();

    let accounts = vec![
        AccountMeta::new(ctx.accounts.source_liquidity.key(), false),
        AccountMeta::new(ctx.accounts.destination_collateral.key(), false),
        AccountMeta::new(ctx.accounts.reserve.key(), false),
        AccountMeta::new(ctx.accounts.reserve_liquidity_supply.key(), false),
        AccountMeta::new(ctx.accounts.reserve_collateral_mint.key(), false),
        AccountMeta::new_readonly(ctx.accounts.lending_market.key(), false),
        AccountMeta::new_readonly(ctx.accounts.lending_market_authority.key(), false),
        AccountMeta::new_readonly(ctx.accounts.user_transfer_authority.key(), true),
        AccountMeta::new_readonly(ctx.accounts.token_program_id.key(), false),
    ];

    let mut data = vec![DEPOSIT_RESERVE_LIQUIDITY];
    data.extend_from_slice(&amount.to_le_bytes());

    let ix = Instruction {
        program_id: lending_program_id,
        accounts,
        data,
    };

    invoke(
        &ix,
        &[
            ctx.accounts.source_liquidity.to_account_info(),
            ctx.accounts.destination_collateral.to_account_info(),
            ctx.accounts.reserve.to_account_info(),
            ctx.accounts.reserve_liquidity_supply.to_account_info(),
            ctx.accounts.reserve_collateral_mint.to_account_info(),
            ctx.accounts.lending_market.to_account_info(),
            ctx.accounts.lending_market_authority.to_account_info(),
            ctx.accounts.user_transfer_authority.to_account_info(),
            ctx.accounts.token_program_id.to_account_info(),
        ],
    );

    msg!("cpi:deposit_reserve_liquidity");

    Ok(())
}

#[derive(Accounts)]
pub struct DepositReserveLiquidity<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Checked in the lending program
    #[account(mut)]
    pub source_liquidity: AccountInfo<'info>,
    /// CHECK: Checked in the lending program
    #[account(mut)]
    pub destination_collateral: AccountInfo<'info>,
    /// CHECK: Checked in the lending program
    #[account(mut,
       owner = lending_program_id.key(),
    )]
    pub reserve: AccountInfo<'info>,
    /// CHECK: Checked in the lending program
    #[account(mut)]
    pub reserve_liquidity_supply: AccountInfo<'info>,
    /// CHECK: Checked in the lending program
    #[account(mut)]
    pub reserve_collateral_mint: AccountInfo<'info>,
    /// CHECK: Checked in the lending program
    #[account(
        owner = lending_program_id.key(),
    )]
    pub lending_market: AccountInfo<'info>,
    /// CHECK: Checked in the lending program
    pub lending_market_authority: AccountInfo<'info>,
    pub user_transfer_authority: Signer<'info>,
    //  pub clock: Sysvar<'info, Clock>,
    /// CHECK: Checked in the lending program
    pub token_program_id: AccountInfo<'info>,
    /// CHECK: Lending program ID
    pub lending_program_id: AccountInfo<'info>,
}
