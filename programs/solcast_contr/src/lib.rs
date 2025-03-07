use anchor_lang::prelude::*;

declare_id!("HH4iTFM56nf5zKPw3LrPZm5EJriyxYwEKMc3eacfqcJr");

#[program]
pub mod solcast_contr {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
