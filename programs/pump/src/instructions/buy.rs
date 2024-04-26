use {
    crate::{calculate_tokens_bought, constants::{INITIAL_PRICE, PRICE_SLOPE, SCALE}, errors::ProgramErrorCode}, anchor_lang::prelude::*, anchor_spl::{
        associated_token::AssociatedToken,
        token::{mint_to, Mint, MintTo, Token, TokenAccount,},
    }, pump_game::state::{admin_config::AdminConfig, game_data::GameData}, solana_program::system_instruction
};
use pump_game;
use anchor_lang::solana_program::sysvar;



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
        seeds = [b"mm"],
        bump
    )]
    pub mint_account: Account<'info, Mint>,

    pub pump_program: Program<'info, pump_game::program::PumpGame>,

    #[account(mut)]
    pub game_data: Account<'info, GameData>,

    /// CHECK:checked in the constraint
    pub admin_data: Account<'info, AdminConfig>,

    #[account(address = sysvar::instructions::id())]
    /// CHECK:checked in the constraint
    instructions: AccountInfo<'info>,

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

    let tokens_bought = calculate_tokens_bought(ctx.accounts.mint_account.supply as f64, sol_amount as f64, 6);
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
    let seeds = b"mm";
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
        tokens_bought , // Mint tokens, adjust for decimals
    )?;

    msg!("Token Bought successfully.");

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

