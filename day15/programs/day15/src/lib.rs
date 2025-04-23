use anchor_lang::prelude::*;

declare_id!("GHwzosMGSPNnCqPEQaccBQNm4a3RtRDJjdy8NrCwHjFv");

#[program]
pub mod day15 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mut a = Vec::new(); {
            for i in 0..50 {
                a.push(i);
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {
}
