use anchor_lang::prelude::*;

pub const MAXIMUM_AGE: u64 = 15;

pub const RESERVE_SWAP_METADATA: usize = 8 + 32 + 32 + 32 + 2 + 100;
pub const SEED_SWAP_METADATA: &[u8] = b"swap_metadata";

#[account]
#[derive(Default)]
pub struct SwapMetadata {
    pub admin: Pubkey,
    pub mint_incoming: Pubkey,
    pub pyth_feed_id: [u8; 32],
    pub discount_bps: u16,
}