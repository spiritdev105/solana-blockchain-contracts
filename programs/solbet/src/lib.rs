use anchor_lang::prelude::*;

declare_id!("ARbowygvyWvish2HNEwsGexXtFHy1ujMAk36sjdSBRrN");

#[program]
pub mod solbet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
