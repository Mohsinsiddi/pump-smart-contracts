use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
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

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        buy::mint_token(ctx, amount)
    }
}
