use {
    crate::{constants::{INITIAL_PRICE, PRICE_SLOPE, SCALE}, errors::ProgramErrorCode}, anchor_lang::prelude::*, anchor_spl::{
        associated_token::AssociatedToken,
        token::{mint_to, Mint, MintTo, Token, TokenAccount,},
    }
};
use solana_program::system_instruction;


#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Address receving the lamports
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    // Mint account address is a PDA
    #[account(
        mut,
        seeds = [b"mi"],
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

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn buy_tokens(ctx: Context<BuyTokens>, sol_amount: u64) -> Result<()> {

    let tokens_bought = calculate_tokens_bought(ctx.accounts.mint_account.supply , sol_amount);
    msg!("Tokens bought with {} SOL: {}", sol_amount, tokens_bought);
   
    let from_account = &ctx.accounts.payer;
        let to_account = &ctx.accounts.recipient;

    // Create the transfer instruction
    let transfer_instruction = system_instruction::transfer(from_account.key, to_account.key, sol_amount);

    // Invoke the transfer instruction
    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            from_account.to_account_info(),
            to_account.clone(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;


    // PDA signer seeds
    let seeds = b"mi";
    let bump = ctx.bumps.mint_account;
    let signer_seeds: &[&[&[u8]]] = &[&[seeds, &[bump]]];

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
        tokens_bought * 10u64.pow(ctx.accounts.mint_account.decimals as u32), // Mint tokens, adjust for decimals
    )?;

    msg!("Token minted successfully.");

    Ok(())
}

// Function to calculate the price of tokens based on the current token supply
fn calculate_token_price(current_supply: u64) -> u64 {
    // Convert INITIAL_PRICE to u128 before multiplication to avoid overflow
    let initial_price = INITIAL_PRICE as u128;
    // Multiply by SCALE before division to maintain precision
    let price_slope_scaled = (current_supply as u128 * PRICE_SLOPE as u128) / SCALE;
    ((initial_price * SCALE as u128) + price_slope_scaled) as u64
}

// Function to calculate SOL needed to buy x amount of tokens
fn calculate_sol_needed(current_supply: u64, desired_token_amount: u64) -> u64 {
    let token_price = calculate_token_price(current_supply);
    // Multiply by SCALE before division to maintain precision
    (desired_token_amount as u128 * token_price as u128 / SCALE) as u64
}

// Function to calculate tokens bought with x amount of SOL
fn calculate_tokens_bought(current_supply: u64, sol_sent: u64) -> u64 {
    let token_price = calculate_token_price(current_supply);
    // Multiply by SCALE before division to maintain precision
    (sol_sent as u128 * SCALE / token_price as u128) as u64
}