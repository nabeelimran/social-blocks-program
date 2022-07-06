use {
    anchor_lang::{
      prelude::*,
      system_program,
      solana_program::{
        program::{
          invoke,
        }
      },
    },
    anchor_spl::{
        associated_token,
        token,
    },
    mpl_token_metadata::{
        ID as TOKEN_METADATA_ID,
        instruction as token_instruction,
    },
};


pub fn mint(
    ctx: Context<MintNft>, 
    metadata_title: String, 
    metadata_symbol: String, 
    metadata_uri: String,
    kind: u8,
    price: u64
) -> Result<()> {

    msg!("Creating mint account...");
    msg!("Mint: {}", &ctx.accounts.mint.key());
    system_program::create_account(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            system_program::CreateAccount {
                from: ctx.accounts.mint_authority.to_account_info(),
                to: ctx.accounts.mint.to_account_info(),
            },
        ),
        10000000,
        82,
        &ctx.accounts.token_program.key(),
    )?;

    msg!("Initializing mint account...");
    msg!("Mint: {}", &ctx.accounts.mint.key());
    token::initialize_mint(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::InitializeMint {
                mint: ctx.accounts.mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        0,
        &ctx.accounts.mint_authority.key(),
        Some(&ctx.accounts.mint_authority.key()),
    )?;

    msg!("Creating token account...");
    msg!("Token Address: {}", &ctx.accounts.token_account.key());    
    associated_token::create(
        CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.mint_authority.to_account_info(),
                associated_token: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
    )?;

    msg!("Minting token to token account...");
    msg!("Mint: {}", &ctx.accounts.mint.to_account_info().key());   
    msg!("Token Address: {}", &ctx.accounts.token_account.key());     
    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
        ),
        1,
    )?;

    msg!("Creating metadata account...");
    msg!("Metadata account address: {}", &ctx.accounts.metadata.to_account_info().key());
    invoke(
        &token_instruction::create_metadata_accounts_v2(
            TOKEN_METADATA_ID, 
            ctx.accounts.metadata.key(), 
            ctx.accounts.mint.key(), 
            ctx.accounts.mint_authority.key(), 
            ctx.accounts.mint_authority.key(), 
            ctx.accounts.mint_authority.key(), 
            metadata_title, 
            metadata_symbol, 
            metadata_uri, 
            None,
            1,
            true, 
            false, 
            None, 
            None,
        ),
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
    )?;

    msg!("Creating master edition metadata account...");
    msg!("Master edition metadata account address: {}", &ctx.accounts.master_edition.to_account_info().key());
    invoke(
        &token_instruction::create_master_edition_v3(
            TOKEN_METADATA_ID, 
            ctx.accounts.master_edition.key(), 
            ctx.accounts.mint.key(), 
            ctx.accounts.mint_authority.key(), 
            ctx.accounts.mint_authority.key(), 
            ctx.accounts.metadata.key(), 
            ctx.accounts.mint_authority.key(), 
            Some(0),
        ),
        &[
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
    )?;

    msg!("Token mint process completed successfully.");

    let post = &mut ctx.accounts.post;

    if kind > 2 {
      return Err(ErrorCode::InvalidType.into())
    }
    if kind > 0 && post.price <= 0 {
      return Err(ErrorCode::ZeroPrice.into())
    }

    if kind == 0 {
      post.price = 0;
    } else {
      post.price = price;
    }
    post.kind = kind;

    post.owner = *ctx.accounts.mint_authority.key;
    post.creator = *ctx.accounts.mint_authority.key;

    Ok(())
}


#[derive(Accounts)]
pub struct MintNft<'info> {
  #[account(init, payer = mint_authority, space = Post::LEN)]
  pub post: Account<'info, Post>,
  /// CHECK: We're about to create this with Metaplex
  #[account(mut)]
  pub metadata: UncheckedAccount<'info>,
  /// CHECK: We're about to create this with Metaplex
  #[account(mut)]
  pub master_edition: UncheckedAccount<'info>,
  #[account(mut)]
  pub mint: Signer<'info>,
  /// CHECK: We're about to create this with Anchor
  #[account(mut)]
  pub token_account: UncheckedAccount<'info>,
  #[account(mut)]
  pub mint_authority: Signer<'info>,
  pub rent: Sysvar<'info, Rent>,
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, token::Token>,
  pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
  /// CHECK: Metaplex will check this
  pub token_metadata_program: UncheckedAccount<'info>,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const KIND_LENGTH: usize = 1;
const PRICE_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;

impl Post {
    const LEN: usize = DISCRIMINATOR_LENGTH
      + PUBLIC_KEY_LENGTH // mint.
      + KIND_LENGTH
      + PRICE_LENGTH
      + PUBLIC_KEY_LENGTH // Owner.
      + PUBLIC_KEY_LENGTH; // Creator.
}

#[account]
pub struct Post {
    pub mint: Pubkey,
    pub kind: u8, // 0: not for sale, 1: sale, 2: auction
    pub price: u64,
    pub owner: Pubkey,
    pub creator: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The type should be 0,1 or 2.")]
    InvalidType,
    #[msg("Price/Base price should be greater than zero.")]
    ZeroPrice
}