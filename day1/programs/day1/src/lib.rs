use anchor_lang::prelude::*;

declare_id!("6ZS8NCEUDUYYBCa23V8KBKv4KfVV5RPY7qJSHTS4Cruq");

#[program]
pub mod day1 {
    use super::*;

    pub fn smthing(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
