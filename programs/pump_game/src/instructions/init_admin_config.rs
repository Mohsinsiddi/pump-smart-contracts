// In this example the same PDA is used as both the address of the mint account and the mint authority
// This is to demonstrate that the same PDA can be used for both the address of an account and CPI signing
use {
    crate::{constants::ADMIN_AUTHORITY, state::admin_config::{AdminConfig, ADMIN_CONFIG_STATE_SIZE}},anchor_lang::prelude::*, 
};


#[derive(Accounts)]
pub struct InitAdminConfigAccount<'info> {
    #[account(mut,address = ADMIN_AUTHORITY)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        space = ADMIN_CONFIG_STATE_SIZE,
        seeds = [b"admin_authority",payer.key().as_ref()],
        bump,
    )]
    pub admin_data: Account<'info, AdminConfig>,

     /// CHECK:checked in the constraint
    pub pump_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}


pub fn init_admin_config_acount(
    ctx: Context<InitAdminConfigAccount>
) -> Result<()> {
    msg!("Creating Admin Config account");
    ctx.accounts.admin_data.pump_program = ctx.accounts.pump_program.key();
    msg!("Admin Config Account created successfully.");
    Ok(())
}

