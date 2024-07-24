use anchor_lang::prelude::*;

declare_id!("AcfG5w5GjU95Ho6imbra9xBW5gvM5ujNdKN7PjNvtkfe");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
