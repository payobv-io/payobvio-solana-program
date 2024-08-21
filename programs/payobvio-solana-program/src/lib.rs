use anchor_lang::prelude::*;

declare_id!("Fu4phdrbvax4SPaQPBcurJFX6YtiqrJLzgh6ryacnwpJ");

#[program]
pub mod payobvio_solana_program {
    use super::*;

    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        bounty_amount: u64,
        issue_id: String,
    ) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        escrow_account.maintainer = ctx.accounts.maintainer.key();
        escrow_account.amount = bounty_amount;
        escrow_account.issue_id = issue_id;
        escrow_account.contributor = Pubkey::default();
        escrow_account.state = EscrowState::Initialized;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bounty_amount: u64, issue_id: String)]
pub struct InitializeEscrow<'info> {
    #[account(mut)]
    pub maintainer: Signer<'info>,
    #[account(
        init,
        payer = maintainer,
        space = 8 + 32 + 32 + 8 + 32 + 1 + issue_id.len(),
        seeds = [b"escrow", issue_id.as_bytes()],
        bump
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EscrowAccount {
    pub maintainer: Pubkey,
    pub contributor: Pubkey,
    pub amount: u64,
    pub issue_id: String,
    pub state: EscrowState,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EscrowState {
    Initialized,
    Funded,
    Assigned,
    Completed,
    Refunded,
}