use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use anchor_lang::solana_program::{lamports, system_instruction};

use crate::instructions::constants::INIT_OBLIGATION;

pub fn init_obligation(ctx: Context<InitObligation>) -> Result<()> {
    let lending_program_id = ctx.accounts.lending_program_id.key(); 

    let accounts = vec![
        AccountMeta::new(ctx.accounts.obligation.key(), false),
        AccountMeta::new_readonly(ctx.accounts.lending_market.key(), false),
        AccountMeta::new_readonly(ctx.accounts.obligation_owner.key(), true),
        AccountMeta::new_readonly(ctx.accounts.rent.key(), false),
        AccountMeta::new_readonly(ctx.accounts.token_program_id.key(), false),
    ];

    let mut data = vec![INIT_OBLIGATION];

    let ix = Instruction {
        program_id: lending_program_id,
        accounts,
        data,
    };

    invoke(
        &ix,
        &[
            ctx.accounts.obligation.to_account_info(),
            ctx.accounts.lending_market.to_account_info(),
            ctx.accounts.obligation_owner.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.token_program_id.to_account_info(),
        ],
    )?;

    msg!("cpi:init_obligation");
    Ok(())
}

#[derive(Accounts)]
pub struct InitObligation<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: checked by lending program
   #[account(mut,
       owner = lending_program_id.key(),
    )]
    pub obligation: AccountInfo<'info>,
    /// CHECK: checked by lending program
    pub lending_market: AccountInfo<'info>,
   // /// CHECK: checked by lending program
    pub obligation_owner: Signer<'info>,
    /// CHECK: checked by lending program
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: checked by lending program
    pub token_program_id: AccountInfo<'info>,
    /// CHECK: Lending program ID
    pub lending_program_id: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

 