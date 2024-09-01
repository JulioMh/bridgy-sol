use anchor_lang::prelude::*;

#[event]
pub struct ReleaseEvent {
    pub to: Pubkey,
    pub amount: u64,
    pub token_address: String,
}

#[account]
pub struct Release {
    pub bump: u8,           // 1
    pub last_claim: u64,    // 32
    pub total_claimed: u64, // 32
}

impl Release {
    pub const LEN: usize = 8 + 1 + 8 + 8;

    pub fn initialize(&mut self, timestamp: u64, bump: u8) {
        self.bump = bump;
        self.total_claimed = 0;
        self.last_claim = timestamp;
    }

    pub fn update_total_claimed(&mut self, rewards: u64, timestamp: u64) {
        self.total_claimed = self.total_claimed.checked_add(rewards).unwrap();
        self.last_claim = timestamp;
    }
}
