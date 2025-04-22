use anchor_lang::prelude::*;

declare_id!("BrabmCLzusrTGLYrFcsNwiVPD6trpQCUbqAmWhCMsM1r");

#[program]
pub mod anchor_function_tutorial {
    use super::*;

    pub fn boaty_mc_boatface(ctx: Context<NonEmpty>, age: u64) -> Result<()> {
        msg!("age is: {}", age);
        Ok(())
    }

    pub fn add(ctx: Context<Empty>, a: u64, b: u64) -> Result<()> {
        msg!("a + b = {}", a+b);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct NonEmpty<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>
}

#[derive(Accounts)]
pub struct Empty {}