use anchor_lang::{prelude::*, solana_program::account_info::Account};
use anchor_spl::token_interface::{Mint, TokenAccount};

declare_id!("7vhGmaff6eXJHREo5RTPvEVUvbE6NySnLY7bjVgtPDj4");

#[program]
pub mod oracle_swap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: InitializeArgs) -> Result<()> {
        ctx.accounts.swap_metadata= data
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, data: SwapArgs) -> Result<()> {
        
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>, // the user of the program (i.e. sending SOL)

    pub mint_incoming: InterfaceAccount<'info, Mint>,

    #[account(seeds = [b"incoming", mint_incoming.key().as_ref()], bump)]
    pub token_account_incoming: InterfaceAccount<'info, TokenAccount>, // program's token account

    #[account(init, payer = admin, space = 8 + 100, seeds=[b"metadata", mint_incoming.key().as_ref()], bump)]
    pub swap_metadata: InterfaceAccount<'info, SwapMetadata>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct InitializeArgs {
    pub mint_incoming: Pubkey, // indicates the identity of the incoming token
    pub discount_bps: u16, // indicates the discount applied to transactions
}

#[account]
#[derive(Default)]
pub struct SwapMetadata {
    pub mint_incoming: Pubkey,
    pub discount_bps: u16,
}

// Defining context for swap function
#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub admin: Signer<'info>, // the user of the program (i.e. sending SOL)

    pub mint_incoming: InterfaceAccount<'info, Mint>,

    #[account(seeds = [b"incoming", mint_incoming.key().as_ref()], bump)]
    pub token_account_incoming: InterfaceAccount<'info, TokenAccount>, // program's token account

    #[account(seeds = [b"metadata", mint_incoming.key().as_ref()], bump)]
    pub swap_metadata: InterfaceAccount<'info, SwapMetadata>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct SwapArgs {
    pub amount_in: u64, // the amount of SOL the user is sending in
    pub oracle_price: u64, // amount of 2Z per SOL
}


// initialize(1 tokens, oracle feed account)
// swap(amountIn, oracle price)