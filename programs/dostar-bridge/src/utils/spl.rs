use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use mpl_token_metadata::instructions::CreateV1CpiBuilder;

pub struct TokenData {
    pub name: String,
    pub uri: String,
    pub symbol: String,
    pub decimals: u8,
}

pub struct CreateTokenAccounts<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub token_metadata_program: UncheckedAccount<'info>,
    pub mint: Account<'info, Mint>,
    pub metadata: UncheckedAccount<'info>,
    pub payer: Signer<'info>,
}

pub fn create_token(
    accounts: CreateTokenAccounts,
    token_data: TokenData,
    signer: &[&[&[u8]]],
) -> solana_program::entrypoint::ProgramResult {
    CreateV1CpiBuilder::new(accounts.token_metadata_program.to_account_info().as_ref())
        .authority(accounts.payer.to_account_info().as_ref())
        .mint(accounts.mint.to_account_info().as_ref(), false)
        .update_authority(accounts.payer.to_account_info().as_ref(), true)
        .system_program(accounts.system_program.to_account_info().as_ref())
        .payer(accounts.payer.to_account_info().as_ref())
        .name(String::from(&token_data.name))
        .uri(String::from(token_data.uri))
        .is_mutable(true)
        .decimals(token_data.decimals)
        .symbol(String::from(token_data.symbol))
        .token_standard(mpl_token_metadata::types::TokenStandard::Fungible)
        .metadata(accounts.metadata.to_account_info().as_ref())
        .seller_fee_basis_points(0)
        .sysvar_instructions(accounts.rent.to_account_info().as_ref())
        .invoke_signed(signer)
}
