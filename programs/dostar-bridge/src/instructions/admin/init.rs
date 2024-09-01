use anchor_lang::prelude::*;

use crate::state::Admin;

pub fn init(ctx: Context<InitCtx>, payload: InitPayload) -> Result<()> {
    ctx.accounts.admin.init(payload);
    Ok(())
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct InitPayload {
    pub signer: Pubkey,
    pub fee_wallet: Pubkey,
    pub be: [u8; 64],
}

#[derive(Accounts)]
#[instruction(payload: InitPayload)]
pub struct InitCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
      init,
      space = Admin::LEN,
      payer = payer,
      seeds = [b"ADMIN"],
      bump,
    )]
    pub admin: Account<'info, Admin>,
    pub system_program: Program<'info, System>,
}
