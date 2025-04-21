use anchor_lang::prelude::*;

declare_id!("HnwJYyjPfVMcNnpq9KsB8QroRVDcvJj74iB1hEf9kr1L");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,
        a: u64,
        b: u64,
        msg: String) -> Result<()> {
            msg!("you sent {}, and, {}. You said '{}'", a,b, msg);
        Ok(())
    }

    pub fn array(ctx: Context<Initialize>,
        arr: Vec<u64>
    ) -> Result<()> {
        msg!("your array: {:?}", arr);
        Ok(())
    }

    pub fn underflow_exercise(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let c = a - b;
        msg!("c is {}", c);
        Ok(())
    }

    pub fn calculator(ctx: Context<Initialize>, add: u64, sub: u64, mul: u64, div: u64, sqrt: f64, log_ten: f32) -> Result<()> {
        msg!("Add plus 2: {}", add + 2);
        msg!("Sub minus 2: {}", sub - 2);
        msg!("Mul by 2: {}", mul * 2);
        msg!("Div by 2: {}", div / 2);
        msg!("SQRT: {}", sqrt.sqrt());
        msg!("log10: {}", log_ten.log10());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
