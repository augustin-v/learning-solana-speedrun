use anchor_lang::prelude::*;

declare_id!("EzvWm9KYLu2TNsh4RoztPJ3BbFnYXihbEUKjmNmNBFay");

#[program]
pub mod read {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
