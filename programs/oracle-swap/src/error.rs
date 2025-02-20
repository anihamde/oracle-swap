use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Discount too high")]
    DiscountTooHigh,
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Insufficient incoming token")]
    InsufficientTokenFunds,
    #[msg("Insufficient SOL within swapper for swap")]
    InsufficientSwappingBalance,
}