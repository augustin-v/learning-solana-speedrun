use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("Gk8EtLHWgvjwDBGzZA7kKfmhWsVedFEzfcGpXPTdE9D1");

#[program]
pub mod basic_storage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Set>, x: u64) -> Result<()> {
        msg!("old value = {}", ctx.accounts.my_storage.x);

        ctx.accounts.my_storage.x = x;
        msg!("new value = {}", ctx.accounts.my_storage.x);
        Ok(())
    }

    pub fn print_x(ctx: Context<PrintX>) -> Result<()> {
        msg!("The value of x is {}", ctx.accounts.my_storage.x);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
              payer = signer,
              space = size_of::<MyStorage>() + 8,
              seeds = [],
              bump)]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PrintX<'info> {
    pub my_storage: Account<'info, MyStorage>,
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,
}

#[account]
pub struct MyStorage {
    x: u64,
}