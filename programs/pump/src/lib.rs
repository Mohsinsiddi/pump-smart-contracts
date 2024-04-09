use anchor_lang::prelude::*;
use instructions::*;
pub mod instructions;
declare_id!("DJMMfpsEPB6JSpzakCK9CqBtAjzRhFx7AYNUAqVktmUE");

#[program]
pub mod pump {
    use super::*;

    pub fn create_token(
        ctx: Context<CreateToken>,
        metadata:InitTokenParams
    ) -> Result<()> {
        create::create_token(ctx,metadata)
    }
}
