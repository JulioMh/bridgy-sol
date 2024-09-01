use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct Authority {
    pub signer: Pubkey,
    pub fee_wallet: Pubkey,
}

impl Authority {
    pub const LEN: usize = 32 + 32;
}
