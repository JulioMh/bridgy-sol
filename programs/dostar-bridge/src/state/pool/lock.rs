use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

#[event]
pub struct LockEvent {
    pub from: Pubkey,
    pub to: String,
    pub amount: u64,
    pub other_chain_token_address: String,
    pub token_address: String,
}
