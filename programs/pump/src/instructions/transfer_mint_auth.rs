use {
    crate::constants::TOKEN_CREATOR, anchor_lang::prelude::*, anchor_spl::{
        associated_token::AssociatedToken,
        token::{set_authority, Mint, SetAuthority, Token},
    }, spl_token::instruction::AuthorityType
};



#[derive(Accounts)]
pub struct TransferMintAuth<'info> {
    #[account(mut,address=TOKEN_CREATOR)]
    pub payer: Signer<'info>,
  /// CHECK: Address validated using constraint
    pub new_mint_auth: AccountInfo<'info>,

    // Mint account address is a PDA
    #[account(
        mut,
        seeds = [b"mi"],
        bump
    )]
    pub mint_account: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn transfer_mint_auth(ctx: Context<TransferMintAuth>) -> Result<()> {

    // PDA signer seeds
    let seeds = b"mi";
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
        AuthorityType::MintTokens,
        Some(ctx.accounts.new_mint_auth.key()),
    )?;

    msg!("Mint Authority Updated Successfully.");

    Ok(())
}

