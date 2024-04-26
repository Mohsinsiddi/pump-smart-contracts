// In this example the same PDA is used as both the address of the mint account and the mint authority
// This is to demonstrate that the same PDA can be used for both the address of an account and CPI signing
use {
    crate::state::game_data::GameData, anchor_lang::prelude::*, 
};


#[derive(Accounts)]
pub struct InitGameAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        space = 1000,
        seeds = [b"game_account",payer.key().as_ref()],
        bump,
    )]
    pub game_data: Account<'info, GameData>,

    pub system_program: Program<'info, System>,
}


pub fn init_game_acount(
    ctx: Context<InitGameAccount>,
    chances:u8
) -> Result<()> {
    msg!("Creating Game account");
    msg!("Payer : {}",ctx.accounts.payer.key());
    ctx.accounts.game_data.init_chances = chances;
    msg!("Game Account created successfully.");
    Ok(())
}

