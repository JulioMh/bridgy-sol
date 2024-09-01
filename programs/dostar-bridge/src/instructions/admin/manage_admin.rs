use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::state::{Admin, Errors, Pool};

pub fn set_signer(ctx: Context<ManageAdmin>, signer: Pubkey) -> Result<()> {
    ctx.accounts.admin.set_signer(signer);
    Ok(())
}

pub fn set_be(ctx: Context<ManageAdmin>, be: [u8; 64]) -> Result<()> {
    ctx.accounts.admin.set_be(be);
    Ok(())
}

pub fn set_fee_wallet(ctx: Context<ManageAdmin>, fee_wallet: Pubkey) -> Result<()> {
    ctx.accounts.admin.set_fee_wallet(fee_wallet);
    Ok(())
}

pub fn set_split_fee(ctx: Context<ManageAdminPool>, split_fee: u64) -> Result<()> {
    require!(split_fee <= 100, Errors::InvalidSplitFee);
    ctx.accounts.pool.set_split_fee(split_fee);
    Ok(())
}

#[derive(Accounts)]
pub struct ManageAdminPool<'info> {
    #[account(
      seeds = [b"ADMIN"],
      bump,
    )]
    pub admin: Account<'info, Admin>,
    #[account(mut, constraint = payer.key() == admin.signer.key())]
    pub payer: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
    mut,
    seeds = [b"pool", mint.key().as_ref()],
    bump,
  )]
    pub pool: Account<'info, Pool>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ManageAdmin<'info> {
    #[account(mut, constraint = payer.key() == admin.signer.key())]
    pub payer: Signer<'info>,
    #[account(
      seeds = [b"ADMIN"],
      bump,
    )]
    pub admin: Account<'info, Admin>,
    pub system_program: Program<'info, System>,
}
