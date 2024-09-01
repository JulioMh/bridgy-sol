use anchor_lang::prelude::*;

#[account]
pub struct ProgramToken {
    pub mint: Pubkey,
    pub symbol: String,
    pub uri: String,
}

impl ProgramToken {
    pub fn len(symbol: String, uri: String) -> usize {
        8 + 32 + 4 + symbol.len() + 4 + uri.len()
    }

    pub fn init(&mut self, mint: Pubkey, symbol: String, uri: String) {
        self.mint = mint;
        self.symbol = symbol;
        self.uri = uri;
    }
}
