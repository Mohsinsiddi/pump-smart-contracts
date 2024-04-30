use {
    crate::{calculate_sol_needed, calculate_tokens_bought, constants::{INITIAL_PRICE, PRICE_SLOPE, SCALE, TOKEN_MINT_SEED}, errors::ProgramErrorCode}, anchor_lang::{prelude::*, system_program::{transfer, Transfer}}, anchor_spl::{
        associated_token::AssociatedToken,
        token::{burn, mint_to, Burn, Mint, MintTo, Token, TokenAccount},
    }, pump_game::state::{admin_config::AdminConfig, game_data::GameData}, solana_program::system_instruction, std::ops::Sub
};
use pump_game;
use anchor_lang::solana_program::sysvar;



#[derive(Accounts)]
#[instruction(symbol:String)]
pub struct SellTokens<'info> {
      /// CHECK: This is the token that we want to mint
      // Mint account address is a PDA
      #[account(
        mut,
        seeds = [TOKEN_MINT_SEED,symbol.as_bytes().as_ref(),creator_address.key().as_ref()],
        bump
    )]
    pub mint_account: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"token_vault".as_ref(),mint_account.key().as_ref()],
        bump
    )]
    pub pda: SystemAccount<'info>,

     /// CHECK: Address receving the lamports
     pub creator_address: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
     /// CHECK: This is the token account that we want to mint tokens to
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK: the authority of the mint account
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn sell_tokens(ctx: Context<SellTokens>,symbol:String, token_amount: u64) -> Result<()> {

    let sol_claimed_for_token_sold = calculate_sol_needed((ctx.accounts.mint_account.supply as f64).sub(token_amount as f64), token_amount as f64, 6);
    msg!("{} SOL will be claim for selling {} tokens",sol_claimed_for_token_sold,token_amount);

    let creator_key =&ctx.accounts.creator_address.key();
        // PDA signer seeds
    let seeds = TOKEN_MINT_SEED;
    let bump = ctx.bumps.mint_account;
    let signer_seeds: &[&[&[u8]]] = &[&[seeds,symbol.as_bytes().as_ref(),creator_key.as_ref(), &[bump]]];

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

    let pda = &mut ctx.accounts.pda;
    let signer = &mut ctx.accounts.signer;
    let system_program = &ctx.accounts.system_program;
    let pda_mint_account = &mut ctx.accounts.mint_account;
    let pda_binding = pda_mint_account.key();
    let pda_mint_key_for_seed = pda_binding.as_ref();

    let pda_balance_before = pda.get_lamports();

    let pda_bump = &[ctx.bumps.pda];
    let seeds: &[&[u8]] = &[b"token_vault".as_ref(),pda_mint_key_for_seed, pda_bump];
    let signer_seeds = &[&seeds[..]];

    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: pda.to_account_info(),
                to: signer.to_account_info(),
            },
        ).with_signer(signer_seeds),
        sol_claimed_for_token_sold,
    )?;    

    let pda_balance_after = pda.get_lamports();

    require_eq!(pda_balance_after, pda_balance_before - sol_claimed_for_token_sold);    

    msg!("Token Sold Sucessfully successfully.");

    Ok(())
}

