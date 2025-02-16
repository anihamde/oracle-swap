use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use crate::error::ErrorCode;

pub fn validate_discount(discount_bps: u16) -> Result<()> {
    if discount_bps >= 10000 {
        return Err(ErrorCode::DiscountTooHigh.into());
    }
    Ok(())
}

pub fn get_discounted_price(price: u64, discount_bps: u16) -> u64 {
    price * (10_000 - u64::from(discount_bps)) / 10_000
}

pub fn transfer_lamports(from: &AccountInfo, to: &AccountInfo, amount: u64) -> Result<()> {
    **from.try_borrow_mut_lamports()? -= amount;
    **to.try_borrow_mut_lamports()? += amount;
    Ok(())
}

pub fn get_pyth_price_from_update(
    price_update: &mut PriceUpdateV2, 
    feed_id: [u8; 32], 
    maximum_age: u64
) -> Result<u64> {
    let price = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?.price;
    if price <= 0 {
        return Err(ErrorCode::InvalidPrice.into());
    }
    Ok(price as u64)
}