use anchor_lang::prelude::*;

declare_id!("9Y9GVeGiqvSFdj5zuWJpGKmBFPMv6zFa2csR7FDZyzef");

#[program]
pub mod day4 {
    use super::*;

    pub fn limit_range(ctx: Context<Initialize>, a: u64) -> Result<()> {
        require!(a >= 10, MyError::AisTooSmall);
        require!(a <= 100, MyError::AisTooBig);
        msg!("Passed value: {}", a);
        Ok(())
    }

    pub fn func(ctx: Context<Initialize>) -> Result<()> {
        msg!("This will print");
        return err!(MyError::AlwaysErrors);
    }
}
#[error_code]
pub enum MyError {
    #[msg("a is too small")]
    AisTooSmall,
    #[msg("a is too big")]
    AisTooBig,
    #[msg("Always errors")]
    AlwaysErrors,
}

#[derive(Accounts)]
pub struct Initialize {}
