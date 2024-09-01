use anchor_lang::prelude::*;

use crate::state::{Authority, Errors};

#[account]
pub struct Pool {
    pub bump: u8,
    pub fee: u64,
    pub split_fees: u64,
    pub other_chain_token_address: String,
    pub authority: Authority,
    pub ata: Pubkey,
    pub token: Pubkey,
    pub is_public: bool,
    pub token_symbol: String,
}

impl Pool {
    pub fn size(other_chain_token_address: String, token_symbol: String) -> usize {
        8 + 1
            + 8
            + 8
            + 4
            + other_chain_token_address.len()
            + Authority::LEN
            + 32
            + 32
            + 8
            + 4
            + token_symbol.len()
    }

    pub fn initialize(
        &mut self,
        bump: u8,
        fee: u64,
        split_fees: u64,
        other_chain_token_address: String,
        authority: Authority,
        ata: Pubkey,
        token: Pubkey,
        is_public: bool,
        symbol: String,
    ) {
        self.ata = ata;
        self.fee = fee;
        self.split_fees = split_fees;
        self.token = token;
        self.authority = authority;
        self.bump = bump;
        self.other_chain_token_address = other_chain_token_address;
        self.is_public = is_public;
        self.token_symbol = symbol;
    }

    pub fn require_authority(&self, payer: &Signer) -> Result<()> {
        require_eq!(payer.key(), self.authority.signer, Errors::AdminOnly);
        Ok(())
    }

    pub fn set_split_fee(&mut self, split_fee: u64) {
        self.split_fees = split_fee
    }
    pub fn set_is_public(&mut self, is_public: bool) {
        self.is_public = is_public
    }
    pub fn set_authority(&mut self, authority: Authority) {
        self.authority = authority
    }
    pub fn set_other_chain_token_address(&mut self, other_chain_token_address: String) {
        self.other_chain_token_address = other_chain_token_address
    }
    pub fn set_fee(&mut self, fee: u64) {
        self.fee = fee
    }
}
