use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("7HpWnAgywraajycPn6a9qi4Df9Tc13toR3HLidEdj3qn");

#[program]
pub mod mappings {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, key: u64) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Set>, key: u64, val: u64) -> Result<()> {
        ctx.accounts.val.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct Initialize<'info> {

    #[account(init,
              payer = signer,
              space = size_of::<Val>() + 8,
              seeds=[&key.to_le_bytes().as_ref()],
              bump)]
    val: Account<'info, Val>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[account]
pub struct Val {
    value: u64
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct Set<'info> {
    #[account(mut)]
    val: Account<'info, Val>,
}