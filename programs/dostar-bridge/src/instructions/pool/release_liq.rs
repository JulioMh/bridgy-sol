use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{transfer, Mint, Token, TokenAccount, Transfer}, };

use crate::{
    state::{Admin, Errors, Pool, Release, ReleaseEvent},
    utils::{calculate_lamports,  Coupon},
};

pub fn release_liq(ctx: Context<ReleaseLiq>, payload: ReleaseLiqPayload) -> Result<()> {
    let bump = ctx.accounts.pool.bump;
    let mint = ctx.accounts.mint.key();
    let pool = ctx.accounts.pool.clone();
    let user = ctx.accounts.user.clone();
    
    require!(
        pool.is_public || user.key().eq(&payload.release.to.key()),
        Errors::PrivateBridge
    );

    payload.coupon.verify(&payload.release.serialize(), &ctx.accounts.admin.be)?;
    

    require!(
        ctx.accounts.pool_ata.amount >= payload.release.amount,
        Errors::InsufficientAmount,
    );

    require!(
        payload.release.timestamp > ctx.accounts.release.last_claim,
        Errors::ClaimedAlready
    );

    let signer: &[&[&[u8]]] = &[&[b"pool", mint.as_ref(), &[bump]]];
    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                to: ctx.accounts.user_ata.to_account_info(),
                from: ctx.accounts.pool_ata.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
            },
            signer,
        ),
        calculate_lamports(payload.release.amount, ctx.accounts.mint.decimals),
    )?;

    ctx.accounts.release.update_total_claimed(payload.release.amount, payload.release.timestamp);

    emit!(ReleaseEvent {
        to: ctx.accounts.user.key(),
        amount: payload.release.amount,
        token_address: ctx.accounts.mint.key().to_string(),
    });

    Ok(())
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct ReleaseLiqPayload {
    pub coupon: Coupon,
    pub release: ReleasePayload,
}

#[derive(Accounts)]
#[instruction(payload: ReleaseLiqPayload)]
pub struct ReleaseLiq<'info> {
    #[account(
        seeds = [b"ADMIN"],
        bump,
    )]
    pub admin: Account<'info, Admin>,
    #[account(mut, constraint = user.key() == admin.signer.key())]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [b"pool", mint.key().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,
    #[account(
        init_if_needed,
        payer=user,
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
        space = Release::LEN, 
        seeds = [b"release",user.key().as_ref(), pool.key().as_ref()], 
        bump,
    )]
    pub release: Account<'info, Release>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct ReleasePayload {
    pub amount: u64,
    pub timestamp: u64,
    pub mint: Pubkey,
    pub to: Pubkey,
}

impl ReleasePayload {
    pub fn serialize(&self) -> Vec<u8> {
        self.try_to_vec().unwrap()
    }
}
