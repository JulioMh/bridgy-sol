use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{
    state::{Admin, Errors, LockEvent, Pool},
    utils::calculate_lamports,
};



pub fn split_amount(pool: &Pool, amount: u64, decimals: u8) -> [u64; 3] {
    let amount_as_lamports: u64 = calculate_lamports(amount, decimals);
    let as_fee =amount_as_lamports.checked_mul(pool.fee).unwrap().checked_div(100).unwrap();
    let amount_to_admin = as_fee.checked_mul(pool.split_fees).unwrap().checked_div(100).unwrap();
    let amount_to_authority = as_fee.checked_sub(amount_to_admin).unwrap();

    let to_receive = amount_as_lamports.checked_sub(as_fee).unwrap();
    [to_receive, amount_to_admin,amount_to_authority]
}

pub fn lock_liq(ctx: Context<LockLiq>, payload: LockLiqPayload) -> Result<()> {
    let pool = ctx.accounts.pool.clone();
    let user = ctx.accounts.user.clone();


    require!(
        pool.is_public || user.key().eq(&pool.authority.signer.key()),
        Errors::PrivateBridge
    );

   let [to_receive, amount_to_admin,amount_to_authority] = split_amount(&pool, payload.amount, ctx.accounts.mint.decimals);
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_ata.to_account_info(),
                to: ctx.accounts.pool_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        to_receive,
    )?;

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_ata.to_account_info(),
                to: ctx.accounts.authority_fee_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount_to_authority,
    )?;

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_ata.to_account_info(),
                to: ctx.accounts.admin_fee_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount_to_admin,
    )?;

    emit!(LockEvent {
        from: ctx.accounts.user.key(),
        to: payload.to,
        amount: to_receive.checked_div(10u64.pow(ctx.accounts.mint.decimals.into())).unwrap(),
        other_chain_token_address: ctx.accounts.pool.other_chain_token_address.to_string(),
        token_address: ctx.accounts.mint.key().to_string()
    });

    Ok(())
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct LockLiqPayload {
    pub to: String,
    pub amount: u64,
    pub timestamp: u64,
}

#[derive(Accounts)]
pub struct LockLiq<'info> {
    #[account(
        seeds = [b"ADMIN"],
        bump,
    )]
    pub admin: Box<Account<'info, Admin>>,
    #[account(
        mut,
        seeds = [b"pool", mint.key().as_ref()],
        bump,
    )]
    pub pool: Box<Account<'info, Pool>>,
    pub mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: Fee Wallet account
    #[account(
        mut, 
        constraint = admin_fee.key() == admin.fee_wallet
    )]
    pub admin_fee: AccountInfo<'info>,
    /// CHECK: Fee Wallet account
    #[account(
        mut, 
        constraint = authority_fee.key() == pool.authority.fee_wallet
    )]
    pub authority_fee: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint=mint,
        associated_token::authority=user,
    )]
    pub user_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint=mint,
        associated_token::authority=pool,
    )]
    pub pool_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint=mint,
        associated_token::authority=admin_fee,
    )]
    pub admin_fee_ata: Account<'info, TokenAccount>,
    #[account(
      init_if_needed,
      payer=user,
      associated_token::mint=mint,
      associated_token::authority=authority_fee,
    )]
    pub authority_fee_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
