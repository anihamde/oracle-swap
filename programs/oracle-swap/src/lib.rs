use anchor_lang::{prelude::*, solana_program::account_info::Account};
use anchor_spl::token_interface::{Mint, TokenAccount};

declare_id!("7vhGmaff6eXJHREo5RTPvEVUvbE6NySnLY7bjVgtPDj4");

#[program]
pub mod oracle_swap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    pub mint_incoming: InterfaceAccount<'info, Mint>,

    #[account(seeds = [b"incoming", mint_incoming.key().as_ref()], bump)]
    pub token_account_incoming: InterfaceAccount<'info, TokenAccount>,

    #[account(init, payer = admin, space = 8 + 100)]
    pub swap_metadata: InterfaceAccount<'info, SwapMetadata>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct InitializeArgs {
    pub discount_bps: u16,
}

#[account]
#[derive(Default)]
pub struct SwapMetadata {
    pub mint_incoming: Pubkey,
    pub discount_bps: u16,
}


// initialize(1 tokens, oracle feed account)
// swap(amountIn, oracle price)