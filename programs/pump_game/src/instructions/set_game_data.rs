use {
    crate::{errors::ProgramErrorCode, state::{admin_config::AdminConfig, game_data::GameData}}, anchor_lang::prelude::*, 
};
use anchor_lang::solana_program::sysvar;



#[derive(Accounts)]
pub struct SetGameAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"game_account",payer.key().as_ref()],
        bump,
    )]
    pub game_data: Account<'info, GameData>,

    #[account(address = sysvar::instructions::id())]
    /// CHECK:checked in the constraint
    instructions: AccountInfo<'info>,

    pub admin_config_data: Account<'info, AdminConfig>,
}


pub fn set_game_data(
    ctx: Context<SetGameAccount>,
    chances:u8
) -> Result<()> {
    let instructions = &mut ctx.accounts.instructions;
    let cpi_program_to_check = sysvar::instructions::get_instruction_relative(0, &instructions)?;

    msg!("program_to_check id {}",cpi_program_to_check.program_id.key());
    require!(
        cpi_program_to_check.program_id.key() != ctx.program_id.key(),
        ProgramErrorCode::NotACPICall
    );

    require!(
        ctx.accounts.admin_config_data.pump_program.key() == cpi_program_to_check.program_id.key(),
        ProgramErrorCode::NotPumpCPICall
     );

    msg!("Setting Game account");
    msg!("payer {}",ctx.accounts.payer.key());
    msg!("prgram id {}",ctx.program_id.key());
    ctx.accounts.game_data.init_chances = chances;
    msg!("Game Account Data updated successfully.");

 
    Ok(())
}

