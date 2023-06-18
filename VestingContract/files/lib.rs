use anchor_lang::prelude::*;
declare_id!("");

#[program]
pub mod Vesting_Contract {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, expiry: i64) -> Result<()> {
        let vesting = &mut ctx.accounts.vesting;
        vesting.expiry = expiry;
        Ok(())
    }
}
