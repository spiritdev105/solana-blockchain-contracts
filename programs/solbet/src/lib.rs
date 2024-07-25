use anchor_lang::prelude::*;

declare_id!("9R8wv2sgJN6GjDtpp73K5zEMT85jZAvd9C3G1ykvkqG9");

#[program]
pub mod solbet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: UserInfo) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let user_data = &mut ctx.accounts.data;
        user_data.name = data.name;
        user_data.age = data.age;
        Ok(())
    }
}

#[account]
pub struct UserInfo {
    pub name: String,
    pub age: u8,
}
const PDA_SEED: &[u8] = b"hello";

#[derive(Accounts)]
#[instruction(instruction_data: UserInfo)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [PDA_SEED, authority.key().as_ref()],
        bump,
        payer = authority,
        space =  8 + 4 + instruction_data.name.len() + 1,
    )]
    pub data: Account<'info, UserInfo>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
