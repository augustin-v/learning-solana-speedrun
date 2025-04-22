use anchor_lang::prelude::*;

declare_id!("Fm7RqcKMgUMRiTrtDZTEViJ4upuDjNukwvjfJSN9gSRv");

#[program]
pub mod sysvar {
    use chrono::*;
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let clock: Clock = Clock::get()?;
        msg!("Current time {}", clock.unix_timestamp);
        Ok(())
    }

    pub fn get_day_of_the_week(_ctx: Context<Initialize>) -> Result<()> {
        let timestamp = Clock::get()?.unix_timestamp;
        let date_time = DateTime::from_timestamp(timestamp, 0).unwrap();
        msg!("today it is: {}", date_time.weekday());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
