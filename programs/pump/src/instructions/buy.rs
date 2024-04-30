use {
    crate::{calculate_sol_needed, calculate_tokens_bought, constants::{INITIAL_PRICE, PRICE_SLOPE, SCALE, TOKEN_MINT_SEED}, errors::ProgramErrorCode}, anchor_lang::{prelude::*, system_program::{transfer, Transfer}}, anchor_spl::{
        associated_token::AssociatedToken,
        token::{mint_to, Mint, MintTo, Token, TokenAccount,},
    }, pump_game::state::{admin_config::AdminConfig, game_data::GameData}, solana_program::system_instruction
};
use pump_game;
use anchor_lang::solana_program::sysvar;



#[derive(Accounts)]
#[instruction(symbol:String)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // Mint account address is a PDA
    #[account(
        mut,
        seeds = [TOKEN_MINT_SEED,symbol.as_bytes().as_ref(),creator_address.key().as_ref()],
        bump
    )]
    pub mint_account: Account<'info, Mint>,

    // Create Associated Token Account, if needed
    // This is the account that will hold the minted tokens
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,


    #[account(
        mut,
        seeds = [b"token_vault".as_ref(),mint_account.key().as_ref()],
        bump
    )]
    pub pda: SystemAccount<'info>,

    /// CHECK: Address receving the lamports
    pub creator_address: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn buy_tokens(ctx: Context<BuyTokens>, symbol:String,sol_amount: u64,) -> Result<()> {

    let prev_tokens_bought = calculate_tokens_bought(ctx.accounts.mint_account.supply as f64, sol_amount as f64, 6);

    let post_tokens_bought = calculate_tokens_bought(ctx.accounts.mint_account.supply as f64 + prev_tokens_bought as f64, sol_amount as f64, 6);
    msg!("Tokens Amount: {} for {} lamports",post_tokens_bought,sol_amount);
    let pda = &mut ctx.accounts.pda;
    let signer = &mut ctx.accounts.payer;
    let system_program = &ctx.accounts.system_program;
    
    let pda_balance_before = pda.get_lamports();
    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: signer.to_account_info(),
                to: pda.to_account_info(),
            },
        ),
        sol_amount,
    )?;

    let pda_balance_after = pda.get_lamports();

    require_eq!(pda_balance_after, pda_balance_before + sol_amount);
    msg!("SOL sent successfully to PDA: {}",pda.key());

    let creator_key =&ctx.accounts.creator_address.key();

    // PDA signer seeds
    let seeds: &[u8; 3] = TOKEN_MINT_SEED;
    let bump = ctx.bumps.mint_account;
    let signer_seeds: &[&[&[u8]]] = &[&[seeds,symbol.as_bytes().as_ref(),creator_key.as_ref(), &[bump]]];

    // Invoke the mint_to instruction on the token program
    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.mint_account.to_account_info(), // PDA mint authority, required as signer
            },
        )
        .with_signer(signer_seeds), // using PDA to sign
        post_tokens_bought , // Mint tokens, adjust for decimals
    )?;

    msg!("Token Bought successfully.");

    Ok(())
}

