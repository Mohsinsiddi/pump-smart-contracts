use {
    crate::{calculate_sol_needed, calculate_tokens_bought, constants::{INITIAL_PRICE, PRICE_SLOPE, SCALE}, errors::ProgramErrorCode}, anchor_lang::prelude::*, anchor_spl::{
        associated_token::AssociatedToken,
        token::{burn, mint_to, Burn, Mint, MintTo, Token, TokenAccount},
    }, pump_game::state::{admin_config::AdminConfig, game_data::GameData}, solana_program::system_instruction
};
use pump_game;
use anchor_lang::solana_program::sysvar;



#[derive(Accounts)]
pub struct SellTokens<'info> {
      /// CHECK: This is the token that we want to mint
      // Mint account address is a PDA
      #[account(
        mut,
        seeds = [b"mm"],
        bump
    )]
    pub mint_account: Account<'info, Mint>,
      pub token_program: Program<'info, Token>,
      /// CHECK: This is the token account that we want to mint tokens to
      #[account(mut)]
      pub from: AccountInfo<'info>,
      /// CHECK: the authority of the mint account
      pub signer: Signer<'info>,
}

pub fn sell_tokens(ctx: Context<SellTokens>, token_amount: u64) -> Result<()> {

    let tokens_sold_for = calculate_sol_needed(ctx.accounts.mint_account.supply as f64, token_amount as f64, 6);
    msg!("Tokens sold for {}", tokens_sold_for);

        // PDA signer seeds
    let seeds = b"mm";
    let bump = ctx.bumps.mint_account;
    let signer_seeds: &[&[&[u8]]] = &[&[seeds, &[bump]]];

    // Invoke the mint_to instruction on the token program
    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint_account.to_account_info(),
                from: ctx.accounts.from.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(), // PDA mint authority, required as signer
            },
        ).with_signer(signer_seeds),
        token_amount , // Mint tokens, adjust for decimals
    )?;

    msg!("Token Sold Sucessfully successfully.");

    // let cpi_context = CpiContext::new(
    //     ctx.accounts.pump_program.to_account_info(), 
    //     pump_game::cpi::accounts::SetGameAccount{
    //     payer:ctx.accounts.payer.to_account_info(),
    //     game_data: ctx.accounts.game_data.to_account_info(),
    //     instructions:ctx.accounts.instructions.to_account_info(),
    //     admin_config_data:ctx.accounts.admin_data.to_account_info()
    //     },
    // );

    // let chances :u8 = 25;

    // pump_game::cpi::set_game_data(
    //     cpi_context,
    //     chances
    // )?;
    // msg!("cpi successful");


    Ok(())
}

