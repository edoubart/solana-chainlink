use anchor_lang::prelude::*;

declare_id!("9cYRL79dw32gpoimWApf8B1apvs6esD9nnExSFKWiioK");

#[program]
pub mod solana_chainlink {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
