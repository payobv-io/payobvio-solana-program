use anchor_lang::prelude::*;

declare_id!("FQvAzGRC6eSDaa1GvuXRDnbBxAxCT8WhwLsj8psnWxSk");

#[program]
pub mod payobvio_solana_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
