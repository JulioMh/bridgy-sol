use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::state::{Authority, Errors, Pool};

pub fn set_is_public(ctx: Context<ManagePool>, is_public: bool) -> Result<()> {
    ctx.accounts.pool.set_is_public(is_public);
    Ok(())
}

pub fn set_authority(ctx: Context<ManagePool>, authority: Authority) -> Result<()> {
    ctx.accounts.pool.set_authority(authority);
    Ok(())
}

pub fn set_other_chain_address(
    ctx: Context<ManagePool>,
    other_chain_token_address: String,
) -> Result<()> {
    ctx.accounts
        .pool
        .set_other_chain_token_address(other_chain_token_address);
    Ok(())
}

pub fn set_fee(ctx: Context<ManagePool>, fee: u64) -> Result<()> {
    require!(fee <= 100, Errors::InvalidFee);
    ctx.accounts.pool.set_fee(fee);
    Ok(())
}

#[derive(Accounts)]
pub struct ManagePool<'info> {
    pub mint: Account<'info, Mint>,
    #[account(
    mut,
    seeds = [b"pool", mint.key().as_ref()],
    bump,
  )]
    pub pool: Account<'info, Pool>,
    #[account(mut, constraint = payer.key() == pool.authority.signer.key())]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
