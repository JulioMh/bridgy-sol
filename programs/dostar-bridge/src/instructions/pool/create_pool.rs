use crate::state::{Admin, Authority, Errors, Pool, ProgramToken};
use crate::utils::calculate_lamports;
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer_checked, TransferChecked};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

pub fn create_pool(ctx: Context<Initialize>, payload: CreatePoolPayload) -> Result<()> {
    require!(payload.fee <= 100, Errors::InvalidFee);
    require!(payload.split_fee <= 100, Errors::InvalidFee);

    let token_program = ctx.accounts.token_program.clone();
    let pool_ata = ctx.accounts.pool_ata.clone();
    let user_ata = ctx.accounts.user_ata.clone();
    let mint = ctx.accounts.mint.clone();
    let payer = ctx.accounts.payer.clone();

    transfer_checked(
        CpiContext::new(
            token_program.to_account_info(),
            TransferChecked {
                from: user_ata.to_account_info(),
                mint: mint.to_account_info(),
                to: pool_ata.to_account_info(),
                authority: payer.to_account_info(),
            },
        ),
        calculate_lamports(payload.amount, ctx.accounts.mint.decimals),
        mint.decimals,
    )?;

    ctx.accounts.pool.initialize(
        ctx.bumps.pool,
        payload.fee,
        payload.split_fee,
        payload.token_address,
        payload.authority,
        pool_ata.key(),
        mint.key(),
        payload.is_public,
        ctx.accounts.token.symbol.clone(),
    );

    Ok(())
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct CreatePoolPayload {
    timestamp: u64,
    amount: u64,
    fee: u64,
    split_fee: u64,
    token_address: String,
    authority: Authority,
    is_public: bool,
}

#[derive(Accounts)]
#[instruction(payload: CreatePoolPayload)]
pub struct Initialize<'info> {
    #[account(mut, constraint = payer.key() == admin.signer.key())]
    pub payer: Signer<'info>,
    #[account(seeds=[b"ADMIN"], bump)]
    pub admin: Account<'info, Admin>,
    #[account(seeds=[b"token", mint.key().as_ref()], bump)]
    pub token: Account<'info, ProgramToken>,
    pub mint: Account<'info, Mint>,
    #[account(
      init,
      payer = payer,
      seeds = [b"pool", mint.key().as_ref()],
      bump,
      space=Pool::size(payload.token_address, token.symbol.clone())
    )]
    pub pool: Account<'info, Pool>,
    #[account(
      init,
      payer = payer,
      associated_token::mint = mint,
      associated_token::authority = pool
    )]
    pub pool_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
      associated_token::mint = mint,
      associated_token::authority = payer
    )]
    pub user_ata: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
