// In this example the same PDA is used as both the address of the mint account and the mint authority
// This is to demonstrate that the same PDA can be used for both the address of an account and CPI signing
use {
    crate::constants::{FEE_RECEIPIENT, MIN_RENT_FOR_TOKEN_PDA, TOKEN_CREATE_FEE, TOKEN_MINT_SEED}, anchor_lang::{prelude::*, system_program::{self, transfer, Transfer}}, anchor_spl::{
        metadata::{create_metadata_accounts_v3, mpl_token_metadata::types::{Creator, DataV2}, CreateMetadataAccountsV3},
        token::{Mint, Token},
    }, solana_program::system_instruction
};


#[derive(Accounts)]
#[instruction(params: InitTokenParams)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Address receving the lamports
    #[account(mut, address = FEE_RECEIPIENT)]
    pub recipient: AccountInfo<'info>,

    // Create mint account
    // Same PDA as address of the account and mint/freeze authority
    #[account(
        init,
        seeds = [TOKEN_MINT_SEED,params.symbol.as_bytes().as_ref(),payer.key().as_ref()],
        bump,
        payer = payer,
        mint::decimals = params.decimals,
        mint::authority = mint_account.key(),
    )]
    pub mint_account: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"token_vault".as_ref(),mint_account.key().as_ref()],
        bump
    )]
    pub pda: SystemAccount<'info>,
    
  /// CHECK: Address validated using constraint
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

    /// CHECK: account constraint checked in account trait
    // #[account(address = mpl_token_metadata::ID)]
    pub token_metadata_program: UncheckedAccount<'info>,
}

// 5. Define the init token params
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitTokenParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
}


pub fn create_token(
    ctx: Context<CreateToken>,
    metadata: InitTokenParams
) -> Result<()> {
    msg!("Creating metadata account");

    let from_account = &ctx.accounts.payer;
    let to_account = &ctx.accounts.recipient;

    // Create the transfer instruction
    let transfer_instruction = system_instruction::transfer(from_account.key, to_account.key, TOKEN_CREATE_FEE);

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
    // We use a PDA as a mint authority for the metadata account because 
    let payer_key = &ctx.accounts.payer.key();
    let name = metadata.name.clone();
    let symbol = metadata.symbol.clone();
    let symbol_seed = symbol.as_bytes();
    let pda_bump = &[ctx.bumps.mint_account];
    let seeds: &[&[u8]] = &[TOKEN_MINT_SEED.as_ref(),symbol_seed.as_ref(),payer_key.as_ref(), pda_bump];
    let signer_seeds:&[&[&[u8]]]  = &[&seeds[..]];

    // Cross Program Invocation (CPI) signed by PDA
    // Invoking the create_metadata_account_v3 instruction on the token metadata program
    create_metadata_accounts_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                mint_authority: ctx.accounts.mint_account.to_account_info(), // PDA is mint authority
                update_authority: ctx.accounts.mint_account.to_account_info(), // PDA is update authority
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        )
        .with_signer(signer_seeds),
        DataV2 {
            name: metadata.name,
            symbol: metadata.symbol,
            uri: metadata.uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        false, // Is mutable
        true,  // Update authority is signer
        None // Collection details
    )?;

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.pda.to_account_info(),
            },
        ).with_signer(signer_seeds),
        MIN_RENT_FOR_TOKEN_PDA,
    )?;    

    msg!("Token created successfully.");

    // Emit event
   
    emit!(TokenCreateEvent {
       creator_address:from_account.key(),
       mint_account:ctx.accounts.mint_account.key(),
       symbol:symbol,
       name:name
    });

    Ok(())
}

#[event]
pub struct TokenCreateEvent {
    pub mint_account: Pubkey,
    pub creator_address: Pubkey,
    pub name: String,
    pub symbol: String,
}
