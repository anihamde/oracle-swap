use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Discount too high")]
    DiscountTooHigh,
    #[msg("Invalid price")]
    InvalidPrice,
}