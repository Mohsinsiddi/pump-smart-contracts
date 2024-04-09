use {
    crate::constants::{DECIMAL, INITIAL_PRICE, SCALE}, anchor_lang::prelude::*, anchor_spl::{
        associated_token::AssociatedToken,
        token::{mint_to, Mint, MintTo, Token, TokenAccount,},
    }
};
use solana_program::system_instruction;


#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Address receving the lamports
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    // Mint account address is a PDA
    #[account(
        mut,
        seeds = [b"mint"],
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

pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
    msg!("Minting token to associated token account...");
    msg!("Mint: {}", &ctx.accounts.mint_account.key());
    msg!(
        "Token Address: {}",
        &ctx.accounts.associated_token_account.key()
    );


    msg!("Solana Amount: {}",amount);

    let _token_amount = how_many_token_sol_can_buy(ctx.accounts.mint_account.supply,amount);
    msg!("Token _token_amount Bought with SOL: {}",_token_amount);
    let _amount = token_to_solbuy(ctx.accounts.mint_account.supply,_token_amount);
   
    msg!("With SOL Token Bought: {}",_amount);
    let from_account = &ctx.accounts.payer;
        let to_account = &ctx.accounts.recipient;

    // Create the transfer instruction
    let transfer_instruction = system_instruction::transfer(from_account.key, to_account.key, amount);

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
    let seeds = b"mint";
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
        amount * 10u64.pow(ctx.accounts.mint_account.decimals as u32), // Mint tokens, adjust for decimals
    )?;

    msg!("Token minted successfully.");

    Ok(())
}

 fn token_to_solbuy( current_token_supply_in_wei:u64,new_token_amount_to_buy:u64) -> u64 {
        let _token_supply_in_ether = current_token_supply_in_wei / SCALE;

        let _a = (new_token_amount_to_buy * DECIMAL) / 2;
        let _b = 2 * _token_supply_in_ether;
        let _c = new_token_amount_to_buy + 1;

        return ((_a * (_b + _c)) / DECIMAL) * SCALE;
}

fn how_many_token_sol_can_buy(current_supply_in_lamports: u64, deposited_sol_amount: u64) -> u64 {
    let mut left = 0;
    let mut right = deposited_sol_amount / INITIAL_PRICE; // Initial upper bound estimation
    
    while left <= right {
        let mid = left + (right - left) / 2;
        let cost = token_to_solbuy(current_supply_in_lamports, mid);
        
        if cost == deposited_sol_amount {
            return mid;
        } else if cost < deposited_sol_amount {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    
    // Adjust the result to handle the scenario where the exact amount cannot be matched
    if token_to_solbuy(current_supply_in_lamports, right) > deposited_sol_amount {
        return right - 1;
    }
    return right;
}
