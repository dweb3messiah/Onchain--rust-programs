use anchor_lang::prelude::*;

declare_id!("Eg9AduT2uedzVvRPNSYt7ttRU3r7oz56G1WYEiA8Np19");

#[program]
pub mod counter_press {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
