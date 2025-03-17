use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;

use crate::instructions::constants::DEPOSIT_OBLIGATION_COLLATERAL;

pub fn deposit_obligation_collateral(
    ctx: Context<DepositObligationCollateral>,
    amount: u64,
) -> Result<()> {
    let lending_program_id = ctx.accounts.lending_program_id.key();

    let accounts = vec![
        AccountMeta::new(ctx.accounts.source_collateral.key(), false),
        AccountMeta::new(ctx.accounts.destination_collateral.key(), false),
        AccountMeta::new(ctx.accounts.deposit_reserve.key(), false),
        AccountMeta::new(ctx.accounts.obligation.key(), false),
        AccountMeta::new_readonly(ctx.accounts.lending_market.key(), false),
        AccountMeta::new_readonly(ctx.accounts.obligation_owner.key(), true),
        AccountMeta::new_readonly(ctx.accounts.user_transfer_authority.key(), true),
        AccountMeta::new_readonly(ctx.accounts.token_program_id.key(), false),
    ];

    let mut data = vec![DEPOSIT_OBLIGATION_COLLATERAL];
    data.extend_from_slice(&amount.to_le_bytes());

    let ix = Instruction {
        program_id: lending_program_id,
        data,
        accounts,
    };

    invoke(
        &ix,
        &[
            ctx.accounts.source_collateral.to_account_info(),
            ctx.accounts.destination_collateral.to_account_info(),
            ctx.accounts.deposit_reserve.to_account_info(),
            ctx.accounts.obligation.to_account_info(),
            ctx.accounts.lending_market.to_account_info(),
            ctx.accounts.obligation_owner.to_account_info(),
            ctx.accounts.user_transfer_authority.to_account_info(),
            ctx.accounts.token_program_id.to_account_info(),
        ],
    );

    msg!("cpi:deposit_collateral");
    Ok(())
}

#[derive(Accounts)]
pub struct DepositObligationCollateral<'info> {
    /// CHECK: checked by lending program
    #[account(mut)]
    pub source_collateral: AccountInfo<'info>,
    /// CHECK: checked by lending program
    #[account(mut)]
    pub destination_collateral: AccountInfo<'info>,
    /// CHECK: checked by lending program
    #[account(mut)]
    pub deposit_reserve: AccountInfo<'info>,
    /// CHECK: checked by lending program
    #[account(mut)]
    pub obligation: AccountInfo<'info>,
    /// CHECK: checked by lending program
    pub lending_market: AccountInfo<'info>,
    ///CHECK: checked by lending program
    pub obligation_owner: Signer<'info>,
    ///CHECK: checked by lending program
    pub user_transfer_authority: Signer<'info>,
    /// CHECK: checked by lending program
    pub token_program_id: AccountInfo<'info>,
    /// CHECK: Lending program ID
    pub lending_program_id: AccountInfo<'info>,
}
