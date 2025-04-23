use anchor_lang::prelude::*;

declare_id!("DpXGsVnESLc5khjUNU15rjCr5mHGyQmrbVrfvHZQ5wfK");

const OWNER: &str = "BJ1k9XXJF9cSoU8xrQ5PsCjR7gT7wgk7jq53fyu89UcR";

#[program]
pub mod day14 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let signer1: &mut Signer = &mut ctx.accounts.signer1;
        let signer2: &mut Signer = &mut ctx.accounts.signer2;
        let signer3: &mut Signer = &mut ctx.accounts.signer3;
        msg!("Signer1 is: {:?}", signer1);
        msg!("Signer2 is: {:?}", signer2);
        msg!("Signer3 is {:?}", signer3);
        Ok(())
    }

    #[access_control(check(&ctx))]
    pub fn access_control(ctx: Context<OnlyOwner>) -> Result<()> {
        msg!("Bonjour i am the owner");
        Ok(())
    }
}

fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.signer_account.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,
    pub signer3: Signer<'info>,
}

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    signer_account: Signer<'info>,
}

#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function")]
    NotOwner
}