use anchor_lang::prelude::*;

declare_id!("3o3zvpMrVivaQRNyhsGGfi5Cm8Q6NJcFJcwoJNBf8Rkq");

#[program]
pub mod deploy_tutorial {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
