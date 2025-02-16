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
        // first transfer sol from the signer account
        
        let from_account = &ctx.accounts.admin
        let to_account = &ctx.accounts.token_account_incoming

        let sol_from_incoming_tx = solana_program::system_instruction::transfer(
            &from_account.key(), &to_account.key(), data.amount_in
        )

        anchor_lang::solana_program::program::invoke_signed(
            &sol_from_incoming_tx,
            &[
                from_account.to_account_info(),
                to_account.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[]
        )?;

        let exchange_rate = 1; // will have to be changed based on oracle arguments
        let final_amount = data.amount_in * (1 - (ctx.accounts.swap_metadata.discount_bps / 10000)) * exchange_rate

        // need to transfer final_amount number of 2Z from the program's token account to the user's wallet.

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

    // #[account(seeds = [b"wallet"], bump)]
    // pub sol_wallet: InterfaceAccount<'info, TokenAccount>
    
    #[account(seeds = [b"metadata", mint_incoming.key().as_ref()], bump)]
    pub swap_metadata: InterfaceAccount<'info, SwapMetadata>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct SwapArgs {
    pub amount_in: u64, // the amount of SOL the user is sending in, in lamports
    pub oracle_price: u64, // amount of 2Z per SOL
}


// initialize(1 tokens, oracle feed account)
// swap(amountIn, oracle price)