pub mod state;
pub mod error;
pub mod utils;
pub mod token;

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};
use crate::{state::*, utils::*, token::transfer_token_if_needed};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

declare_id!("7vhGmaff6eXJHREo5RTPvEVUvbE6NySnLY7bjVgtPDj4");

#[program]
pub mod oracle_swap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: InitializeArgs) -> Result<()> {
        validate_discount(data.discount_bps)?;

        ctx.accounts.swap_metadata.admin = *ctx.accounts.admin.key;
        ctx.accounts.swap_metadata.mint_incoming = ctx.accounts.mint_incoming.key();
        ctx.accounts.swap_metadata.feed_id_incoming = data.feed_id_incoming;
        ctx.accounts.swap_metadata.discount_bps = data.discount_bps;
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, data: SwapArgs) -> Result<()> {
        let price_sol = get_pyth_price_from_update(&mut ctx.accounts.price_update_sol, SOL_FEED_ID, MAXIMUM_AGE)?;
        let price_sol_discount = get_discounted_price(price_sol, ctx.accounts.swap_metadata.discount_bps);
        let price_incoming = get_pyth_price_from_update(&mut ctx.accounts.price_update_incoming, ctx.accounts.swap_metadata.feed_id_incoming, MAXIMUM_AGE)?;

        let sol_outgoing = data.amount_incoming * price_incoming / price_sol_discount;

        transfer_token_if_needed(
            &ctx.accounts.ta_swapper, 
            &ctx.accounts.ta_program, 
            &ctx.accounts.token_program, 
            &ctx.accounts.swapper, 
            &ctx.accounts.mint_incoming, 
            data.amount_incoming,
        )?;

        transfer_lamports(
            &ctx.accounts.swap_metadata.to_account_info(), 
            &ctx.accounts.swapper.to_account_info(), 
            sol_outgoing,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    pub mint_incoming: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = mint_incoming,
        associated_token::authority = swap_metadata,
        associated_token::token_program = token_program,
    )]
    pub ta_program: InterfaceAccount<'info, TokenAccount>,

    #[account(init, payer = admin, space = RESERVE_SWAP_METADATA, seeds = [SEED_SWAP_METADATA], bump)]
    pub swap_metadata: Account<'info, SwapMetadata>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct InitializeArgs {
    pub discount_bps: u16,
    pub feed_id_incoming: [u8; 32],
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub swapper: Signer<'info>,

    // TODO: allow delegate?
    #[account(
        mut,
        token::mint = mint_incoming,
        token::authority = swapper,
        token::token_program = token_program
    )]
    pub ta_swapper: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_incoming,
        associated_token::authority = swap_metadata,
        associated_token::token_program = token_program,
    )]
    pub ta_program: InterfaceAccount<'info, TokenAccount>,

    #[account(constraint = mint_incoming.key() == swap_metadata.mint_incoming)]
    pub mint_incoming: InterfaceAccount<'info, Mint>,

    pub price_update_sol: Account<'info, PriceUpdateV2>,

    pub price_update_incoming: Account<'info, PriceUpdateV2>,

    #[account(seeds = [SEED_SWAP_METADATA], bump)]
    pub swap_metadata: Account<'info, SwapMetadata>,

    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct SwapArgs {
    pub amount_incoming: u64,
}
