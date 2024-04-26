use {
    crate::{constants::ADMIN_AUTHORITY, errors::ProgramErrorCode, state::admin_config::AdminConfig}, anchor_lang::prelude::*, 
};



#[derive(Accounts)]
pub struct SetAdminConfigAccount<'info> {
    #[account(mut,address = ADMIN_AUTHORITY)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"admin_authority",payer.key().as_ref()],
        bump,
    )]
    pub admin_data: Account<'info, AdminConfig>,
 /// CHECK:checked in the constraint
    pub new_pump_program: AccountInfo<'info>,
}


pub fn set_admin_config_data(
    ctx: Context<SetAdminConfigAccount>,
) -> Result<()> {
    msg!("Setting Admin Account Data");
    ctx.accounts.admin_data.pump_program = ctx.accounts.new_pump_program.key();
    Ok(())
}

