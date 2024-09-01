use anchor_lang::prelude::*;

use instructions::*;
pub mod instructions;
pub mod state;
pub mod utils;
use state::Authority;

declare_id!("5WuuHwzQrKwnRu6FYzDQXzueGBjXog5uHucFcb6TEALe");

#[program]
pub mod dogstar_bridge {

    use super::*;

    // ADMIN
    pub fn init(ctx: Context<InitCtx>, payload: InitPayload) -> Result<()> {
        instructions::init(ctx, payload)
    }

    pub fn set_signer(ctx: Context<ManageAdmin>, payload: Pubkey) -> Result<()> {
        instructions::set_signer(ctx, payload)
    }

    pub fn set_be(ctx: Context<ManageAdmin>, payload: [u8; 64]) -> Result<()> {
        instructions::set_be(ctx, payload)
    }
    pub fn set_fee_wallet(ctx: Context<ManageAdmin>, payload: Pubkey) -> Result<()> {
        instructions::set_fee_wallet(ctx, payload)
    }

    pub fn create_token(ctx: Context<CreateToken>, payload: CreateTokenPayload) -> Result<()> {
        instructions::create_token(ctx, payload)
    }

    // POOL

    pub fn create_pool(ctx: Context<Initialize>, payload: CreatePoolPayload) -> Result<()> {
        instructions::create_pool(ctx, payload)
    }

    pub fn lock_liq(ctx: Context<LockLiq>, payload: LockLiqPayload) -> Result<()> {
        instructions::lock_liq(ctx, payload)
    }

    pub fn release_liq(ctx: Context<ReleaseLiq>, payload: ReleaseLiqPayload) -> Result<()> {
        instructions::release_liq(ctx, payload)
    }

    // POOL AUTHORITY

    pub fn set_is_public(ctx: Context<ManagePool>, payload: bool) -> Result<()> {
        instructions::set_is_public(ctx, payload)
    }

    pub fn set_authority(ctx: Context<ManagePool>, payload: Authority) -> Result<()> {
        instructions::set_authority(ctx, payload)
    }

    pub fn set_other_chain_address(ctx: Context<ManagePool>, payload: String) -> Result<()> {
        instructions::set_other_chain_address(ctx, payload)
    }

    pub fn set_fee(ctx: Context<ManagePool>, payload: u64) -> Result<()> {
        instructions::set_fee(ctx, payload)
    }
}
