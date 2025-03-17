use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;

use crate::instructions::constants::REPAY_OBLIGATION_LIQUIDITY;

pub fn repay_obligation_liquidity(
    ctx: Context<RepayObligationLiquidity>,
    amount: u64,
) -> Result<()> {
    let lending_program_id = ctx.accounts.lending_program_id.key();

    let accounts = vec![
        AccountMeta::new(ctx.accounts.source_liquidity.key(), false),
        AccountMeta::new(ctx.accounts.destination_liquidity.key(), false),
        AccountMeta::new(ctx.accounts.repay_reserve.key(), false),
        AccountMeta::new(ctx.accounts.obligation.key(), false),
        AccountMeta::new_readonly(ctx.accounts.lending_market.key(), false),
        AccountMeta::new_readonly(ctx.accounts.user_transfer_authority.key(), true),
        AccountMeta::new_readonly(ctx.accounts.token_program_id.key(), false),
    ];

    let mut data = vec![REPAY_OBLIGATION_LIQUIDITY];
    data.extend_from_slice(&amount.to_le_bytes());

    let ix = Instruction {
        program_id: lending_program_id,
        data,
        accounts,
    };

    invoke(
        &ix,
        &[
            ctx.accounts.source_liquidity.to_account_info(),
            ctx.accounts.destination_liquidity.to_account_info(),
            ctx.accounts.repay_reserve.to_account_info(),
            ctx.accounts.obligation.to_account_info(),
            ctx.accounts.lending_market.to_account_info(),
            ctx.accounts.user_transfer_authority.to_account_info(),
            ctx.accounts.token_program_id.to_account_info(),
        ],
    );

    msg!("cpi:repay_liquidity");
    Ok(())
}

#[derive(Accounts)]
pub struct RepayObligationLiquidity<'info> {
    /// CHECK: Checked in the lending program
    #[account(mut)]
    pub source_liquidity: AccountInfo<'info>,
    /// CHECK: Checked in the lending program
    #[account(mut)]
    pub destination_liquidity: AccountInfo<'info>,
    /// CHECK: Checked in the lending program
    #[account(mut)]
    pub repay_reserve: AccountInfo<'info>,
    /// CHECK: Checked in the lending program
    #[account(mut)]
    pub obligation: AccountInfo<'info>,
    /// CHECK: Checked in the lending program
    pub lending_market: AccountInfo<'info>,
    /// CHECK: Checked in the lending program
    pub user_transfer_authority: Signer<'info>,
    /// CHECK: Checked in the lending program
    pub token_program_id: AccountInfo<'info>,
    /// CHECK: Checked in the lending program
    pub lending_program_id: AccountInfo<'info>,
}
