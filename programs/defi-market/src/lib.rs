mod instructions;
mod state;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;

use instructions::*;
use state::*;

declare_id!("5hSmUyjTFpeeZScxDxUpLX5rg4GyN1yCh8ky6GfQ44X6");

#[program]
pub mod defi_market {
    use super::*;


    pub fn deposit_reserve_liquidity(
        ctx: Context<DepositReserveLiquidity>,
        amount: u64,
    ) -> Result<()> {
        instructions::deposit_reserve_liquidity(ctx, amount)?;
        Ok(())
     }
    pub fn deposit_obligation_collateral(ctx: Context<DepositObligationCollateral>, amount: u64) -> Result<()> {
        instructions::deposit_obligation_collateral(ctx, amount)?;
        Ok(())
    }

    pub fn borrow_obligation_liquidity(ctx: Context<BorrowObligationLiquidity>, amount: u64) -> Result<()> {
        instructions::borrow_obligation_liquidity(ctx, amount)?;
        Ok(())
    }
    pub fn repay_obligation_liquidity(ctx: Context<RepayObligationLiquidity>, amount: u64) -> Result<()> {
        instructions::repay_obligation_liquidity(ctx, amount)?;
        Ok(())
    }


    pub fn init_obligation(ctx: Context<InitObligation>) -> Result<()> {
        instructions::init_obligation(ctx)?;
        Ok(())
    }

}
