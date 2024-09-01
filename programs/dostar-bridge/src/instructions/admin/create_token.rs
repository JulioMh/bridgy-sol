use crate::state::{Admin, ProgramToken};
use crate::utils::{
    self, calculate_lamports, CreateTokenAccounts, TokenData, MPL_TOKEN_METADATA_ID,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{set_authority, SetAuthority};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};


pub fn create_token(ctx: Context<CreateToken>, payload: CreateTokenPayload) -> Result<()> {
    msg!("{:?}", ctx.accounts.mint);

    let system_program = ctx.accounts.system_program.clone();
    let token_program = ctx.accounts.token_program.clone();
    let rent = ctx.accounts.rent.clone();
    let token_metadata_program = ctx.accounts.token_metadata_program.clone();
    let mint = ctx.accounts.mint.clone();
    let metadata = ctx.accounts.metadata.clone();
    let payer = ctx.accounts.payer.clone();
    let symbol = payload.symbol.clone();

    let signer: &[&[&[u8]]] = &[&[
        b"token",
        payload.authority.as_ref(),
        symbol.as_ref(),
        &[ctx.bumps.mint],
    ]];

    utils::create_token(
        CreateTokenAccounts {
            system_program,
            token_metadata_program,
            token_program: token_program.clone(),
            rent,
            mint: *mint.clone(),
            metadata,
            payer,
        },
        TokenData {
            name: payload.name,
            uri: payload.uri.clone(),
            symbol: payload.symbol,
            decimals: payload.decimals,
        },
        signer,
    )?;

    let total_supply = calculate_lamports(payload.total_supply, payload.decimals);

    if payload.mint_supply == true {
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.payer.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.user_ata.to_account_info(),
                },
                signer,
            ),
            total_supply,
        )?;
    }

    set_authority(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            SetAuthority {
                current_authority: ctx.accounts.payer.to_account_info(),
                account_or_mint: mint.to_account_info(),
            },
            signer,
        ),
        anchor_spl::token::spl_token::instruction::AuthorityType::MintTokens,
        if payload.revoke_authority == true {
            None
        } else {
            Some(ctx.accounts.authority.key())
        },
    )?;

    ctx.accounts.token.init(mint.key(), symbol, payload.uri);

    Ok(())
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct CreateTokenPayload {
    timestamp: u64,
    name: String,
    uri: String,
    symbol: String,
    decimals: u8,
    total_supply: u64,
    revoke_authority: bool,
    mint_supply: bool,
    authority: Pubkey,
}

#[derive(Accounts)]
#[instruction(payload: CreateTokenPayload)]
pub struct CreateToken<'info> {
    #[account(seeds=[b"ADMIN"], bump)]
    pub admin: Box<Account<'info, Admin>>,
    #[account(mut, constraint = admin.signer.key() == payer.key())]
    pub payer: Signer<'info>,
    #[account(
        init,
        seeds=[b"token", payload.authority.as_ref(), payload.symbol.as_ref()],
        bump,
        payer=payer,
        mint::decimals=payload.decimals,
        mint::authority=payer
    )]
    pub mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer=payer,
        seeds=[b"token", mint.key().as_ref()],
        bump, 
        space=ProgramToken::len(payload.symbol, payload.uri)
    )]
    pub token: Box<Account<'info, ProgramToken>>,
    #[account(
      init,
      payer = payer,
      associated_token::mint = mint,
      associated_token::authority = payer
    )]
    pub user_ata: Account<'info, TokenAccount>,
    /// CHECK: Authority
    #[account()]
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: Metaplex Program
    #[account(address = MPL_TOKEN_METADATA_ID)]
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: Metadata account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
}
