use {
    crate::constants::*, anchor_lang::prelude::*, anchor_spl::{
        associated_token::AssociatedToken,
        token::{set_authority, Mint, SetAuthority, Token},
    }, spl_token::instruction::AuthorityType
};



#[derive(Accounts)]
pub struct RevokeFreezeAuth<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // Mint account address is a PDA
    #[account(
        mut,
        seeds = [TOKEN_MINT_SEED],
        bump
    )]
    pub mint_account: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn revoke_freeze_auth(ctx: Context<RevokeFreezeAuth>) -> Result<()> {

    // PDA signer seeds
    let seeds = TOKEN_MINT_SEED;
    let bump = ctx.bumps.mint_account;
    let signer_seeds: &[&[&[u8]]] = &[&[seeds, &[bump]]];

    // Invoke the set_authority instruction on the token program
    set_authority(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            SetAuthority {
                current_authority: ctx.accounts.mint_account.to_account_info(),
                account_or_mint: ctx.accounts.mint_account.to_account_info(),
            },
            signer_seeds
        ),
        AuthorityType::FreezeAccount,
        None
    )?;

    msg!("Mint Authority Updated Successfully.");

    Ok(())
}

