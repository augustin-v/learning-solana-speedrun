use anchor_lang::prelude::*;

declare_id!("9x4Zd75xxRYUWivhD6vRY4fVpbreSyK5ikvtxUXvip6L");

#[program]
pub mod day13 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        emit!(MyEvent { value: 46 });
        emit!(MySecondEvent { value: 41, msg: "Super".to_string() });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[event]
pub struct MyEvent {
    pub value: u64,
}

#[event]
pub struct MySecondEvent {
    pub value: u64,
    pub msg: String,
}