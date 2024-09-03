use anchor_lang::prelude::*;

declare_id!("4DSqjxxgpSdAkMShPrR1yQJ8261w2Ch8TgKg3pYnLseA");

#[program]
pub mod dice_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
