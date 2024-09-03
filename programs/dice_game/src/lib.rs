use anchor_lang::prelude::*;

mod state;
mod instructions;
mod error;

use instructions::*;


declare_id!("AhW6QqMuKF7LTdmUTunnxdxZHsU2P4972zpsPTB5AZww");

#[program]
pub mod dice_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.init(amount)
    }

    pub fn place_bet(ctx: Context<PlaceBet>, amount: u64, seed: u64, roll: u8) -> Result<()> {
        ctx.accounts.init(seed, amount, roll, &ctx.bumps)?;
        ctx.accounts.deposit(amount)
    }

    pub fn resolve_bet(ctx: Context<ResolveBet>, sig:  Vec<u8>) -> Result<()> {
        ctx.accounts.verify_ed25519_signature(&sig)?;
        ctx.accounts.resolve_bet(&sig, &ctx.bumps)
    }

    pub fn refund_bet(ctx: Context<RefundBet>) -> Result<()> {
        ctx.accounts.refund(&ctx.bumps)
    }
}
