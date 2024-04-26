use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod state;
use instructions::*;
pub mod instructions;
declare_id!("AGAtML592JZnHBjDDEZ97DGNy9jwETaPiAxtrBfcXFaX");

#[program]
pub mod pump_game {
    use super::*;

    pub fn init_account(
        ctx: Context<InitGameAccount>,
        chances: u8
    ) -> Result<()> {
        init_account::init_game_acount(ctx, chances)
    }

    pub fn set_game_data(
        ctx: Context<SetGameAccount>,
        chances: u8
    ) -> Result<()> {
        set_game_data::set_game_data(ctx,chances)
    }

    pub fn init_admin_config_account(
        ctx: Context<InitAdminConfigAccount>,
    ) -> Result<()> {
        init_admin_config::init_admin_config_acount(ctx)
    }

    pub fn set_admin_config_data(
        ctx: Context<SetAdminConfigAccount>,
    ) -> Result<()> {
        set_admin_config::set_admin_config_data(ctx)
    }
}
