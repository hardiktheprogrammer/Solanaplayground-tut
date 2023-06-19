use anchor_lang::prelude::*;

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
        if ctx.accounts.clock.unix_timestamp < vesting.expiry {
            return Err(ErrorCode::NotYetExpired.into());
        }
        if vesting.claimed {
            return Err(ErrorCode::TokensAlreadyClaimed.into());
        }
        vesting.claimed = true;

        vesting.claimed_data = ctx.accounts.clock.unix_timestamp;

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
