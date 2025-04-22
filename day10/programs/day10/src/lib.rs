use anchor_lang::prelude::*;

declare_id!("5nKxtn1a587cDSctzpxWGybzFhTFqX5pE1rXEapWuMc8");

#[program]
pub mod day10 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let u = get_a_num();
        msg!("got the num {}", u);
        another_one::another();
        something_smth::private_function();
        Ok(())
    }

    pub mod another_one {
        pub fn another() {}
    }

    pub mod something_smth {
        pub(in crate::day10) fn private_function() {

        }
    }
}

mod whatever {
    use crate::day10;

    pub fn nice() {
        day10::another_one::another();
    }
}

fn get_a_num() -> u64 {
    32
}

#[derive(Accounts)]
pub struct Initialize {}
