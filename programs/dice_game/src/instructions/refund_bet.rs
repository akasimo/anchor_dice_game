use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

use crate::{error::DiceError, state::Bet};

#[derive(Accounts)]
pub struct RefundBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(mut)]
    pub house: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), player.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump = bet.bump
    )]
    pub bet: Account<'info, Bet>,

    pub system_program: Program<'info, System>,
}

impl<'info> RefundBet<'info> {
    pub fn refund(&mut self, bumps: &RefundBetBumps) -> Result<()> {

        let slot = Clock::get()?.slot;

        require!((slot - self.bet.slot) < 324, DiceError::BetNotExpired);

        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.player.to_account_info(),
        };

        let seeds = [
            b"vault",
            &self.house.key().to_bytes()[..],
            &[bumps.vault]
        ];
        let signer_seeds = &[&seeds[..]];
        
        let ctx = CpiContext::new_with_signer(self.system_program.to_account_info(), accounts, signer_seeds);
        transfer(ctx, self.bet.amount)?;

        Ok(())
    }
}