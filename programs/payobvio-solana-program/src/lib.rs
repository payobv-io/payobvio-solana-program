use anchor_lang::prelude::*;

declare_id!("FQvAzGRC6eSDaa1GvuXRDnbBxAxCT8WhwLsj8psnWxSk");

#[program]
pub mod payobvio_solana_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, message: String) -> Result<()> {
        let account = &mut ctx.accounts.data_account;
        account.message = message;
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Message: {:?}", account.message);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 40,
        seeds = [b"data_account".as_ref(), user.key().as_ref()],
        bump
    )]
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub message: String,
}