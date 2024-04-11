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

    pub fn buy_tokens(ctx: Context<BuyTokens>, sol_amount: u64) -> Result<()> {
        buy::buy_tokens(ctx, sol_amount)
    }

    pub fn transfer_mint_auth(ctx:Context<TransferMintAuth>)-> Result<()> {
       transfer_mint_auth::transfer_mint_auth(ctx)
    }

    pub fn revoke_freeze_auth(ctx:Context<RevokeFreezeAuth>)-> Result<()> {
        revoke_freeze_auth::revoke_freeze_auth(ctx)
    }
}
