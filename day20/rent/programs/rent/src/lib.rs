use anchor_lang::prelude::*;
use anchor_lang::solana_program::rent as other_rent;

declare_id!("AzRcxsMdnvjKxJ3CkYn57hkX3jYuRdUh7LXWcGuYLgRK");

#[program]
pub mod rent {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let cost_of_empty_acc = other_rent::ACCOUNT_STORAGE_OVERHEAD as f64 *
                                     other_rent::DEFAULT_LAMPORTS_PER_BYTE_YEAR as f64 *
                                     other_rent::DEFAULT_EXEMPTION_THRESHOLD;
        msg!("cost to create empty account: {}", cost_of_empty_acc);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
