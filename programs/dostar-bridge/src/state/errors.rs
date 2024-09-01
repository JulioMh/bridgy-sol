use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("User does not have enough tokens")]
    InsufficientAmount,
    #[msg("Invalid Coupon")]
    InvalidCoupon,
    #[msg("Admin only")]
    AdminOnly,
    #[msg("Invalid fee")]
    InvalidFee,
    #[msg("Invalid split fee")]
    InvalidSplitFee,
    #[msg("Bridge is private")]
    PrivateBridge,
    #[msg("This coupon already has been used")]
    ClaimedAlready,
}
