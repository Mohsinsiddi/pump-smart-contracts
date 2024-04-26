use anchor_lang::prelude::*;

pub const ADMIN_CONFIG_STATE_SIZE: usize = 8 + 32;

#[account]
pub struct AdminConfig {
    pub pump_program: Pubkey
}

impl AdminConfig {
    pub fn register_pump_program(&mut self, pump_game:Pubkey) -> Result<()> {
        self.pump_program = pump_game;
        Ok(())
    }
}
