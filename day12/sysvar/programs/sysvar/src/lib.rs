use anchor_lang::prelude::*;

declare_id!("7iSXqkbYmj4QojdPsZnHPVdsstgQSFDAgyfkipLWgXqR");

#[program]
pub mod sysvar {
    use anchor_lang::solana_program::last_restart_slot::LastRestartSlot;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let clock = Rent::get()?;
        msg!("rent : {:?}", clock);
        let arr = [ctx.accounts.stake_history.clone()];

        let accounts_iter = &mut arr.iter();

        let sh_sysvar_info = next_account_info(accounts_iter)?;

        let stake_history = StakeHistory::from_account_info(sh_sysvar_info)?;

        let last_restart_slot = LastRestartSlot::get()?;

        msg!("Stake history: {:?}", stake_history);
        msg!("Last restart slot: {:?}", last_restart_slot);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK:
    pub stake_history: AccountInfo<'info>,
    /// CHECK:
    pub last_restart_slot: AccountInfo<'info>,
}
