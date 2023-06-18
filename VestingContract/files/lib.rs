use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};

declare_id!("");

#[program]
pub mod Vesting_Contract {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, expiry: i64) -> Result<()> {
        let vesting = &mut ctx.accounts.vesting;
        vesting.expiry = expiry;
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let vesting = &mut ctx.accounts.vesting;

        vesting.expiry = expiry;

        Ok(())
    }
}
#[derive(Accounts)]

pub struct Initialize<'info> {
    #[account(init, payer = user , space = 8 + 8 ) ]
    pub Vesting: ProgramAccount<'info, Vesting>,

    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]

pub struct Claim<'info> {
    #[account(mut)]
    pub vesting: ProgramAccount<'info, Vesting>,
    pub clock: Sysvar<'info, Clock>,
}
