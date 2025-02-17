use anchor_lang::prelude::*;

pub const MAXIMUM_AGE: u64 = 15;
pub const SOL_FEED_ID: [u8; 32] = [239, 13, 139, 111, 218, 44, 235, 164, 29, 161, 93, 64, 149, 209, 218, 57, 42, 13, 47, 142, 208, 198, 199, 188, 15, 76, 250, 200, 194, 128, 181, 109];

pub const RESERVE_SWAP_METADATA: usize = 8 + 32 + 32 + 32 + 2 + 100;
pub const SEED_SWAP_METADATA: &[u8] = b"swap_metadata";

#[account]
#[derive(Default)]
pub struct SwapMetadata {
    pub admin: Pubkey,
    pub mint_incoming: Pubkey,
    pub feed_id_incoming: [u8; 32],
    pub discount_bps: u16,
}