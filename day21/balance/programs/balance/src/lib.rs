use anchor_lang::prelude::*;

declare_id!("GCew4YjsDzRAhqWoWrr9kvRDEeMW91y6pj63ZkPwwrFS");

#[program]
pub mod balance {
    use super::*;

    pub fn read_balance(ctx: Context<ReadBalance>) -> Result<()> {
        let balance = ctx.accounts.account.to_account_info().lamports();

        msg!("Balance in lamports is {}", balance);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct ReadBalance<'info> {
    /// CHECK: although we read this account's balance, we do not do anything with the information
    pub account: UncheckedAccount<'info>,
}
