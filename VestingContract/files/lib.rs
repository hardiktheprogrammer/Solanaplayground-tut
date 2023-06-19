use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};

declare_id!("");

#[program]
pub mod Vesting_Contract {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, expiry: i64) -> Result<()> {
        let vesting = &mut ctx.accounts.vesting;
        vesting.expiry = expiry;
        vesting.claimed = false;
        vesting.claimed_data = 0;
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

#[account]
pub struct Vesting {
    pub expiry: i64,

    pub claimed: bool,
    pub claimed_data: u64,
}

#[error]
pub enum ErrorCode {
    #[msg("Vesting Period is not over yet expired")]
    NotYetExpired,
    #[msg("Tokens have already been claimed.")]
    TokensAlreadyClaimed,
}
