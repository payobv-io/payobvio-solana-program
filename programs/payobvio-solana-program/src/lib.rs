use anchor_lang::prelude::*;

declare_id!("4bq5yNyzhykT3zTFTwLHZqU3U6n1kv3ko6vvB455TrcA");

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

    pub fn close_escrow(ctx: Context<CloseEscrow>) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
    
        require!(
            escrow_account.state == EscrowState::Initialized,
            EscrowError::InvalidEscrowState
        );
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

#[derive(Accounts)]
pub struct CloseEscrow<'info> {
    #[account(mut)]
    pub maintainer: Signer<'info>,
    #[account(
        mut,
        close = maintainer,
        constraint = escrow_account.maintainer == maintainer.key(),
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
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

#[error_code]
pub enum EscrowError {
    #[msg("The escrow is not in the correct state for this operation")]
    InvalidEscrowState,
    #[msg("The deposit amount does not match the bounty amount")]
    InvalidDepositAmount,
    #[msg("The contributor does not match the assigned contributor")]
    InvalidContributor,
}