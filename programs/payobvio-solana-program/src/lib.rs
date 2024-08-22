use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("DonZJdwWoKHey7iQoskXvbMYJ1xcepzoWTogFnWm4hyE");

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
            escrow_account.state == EscrowState::Funded,
            EscrowError::InvalidEscrowState
        );

        Ok(())
    }

    pub fn deposit_funds(ctx: Context<DepositFunds>, amount: u64) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        let maintainer = &ctx.accounts.maintainer;

        require!(
            escrow_account.state == EscrowState::Initialized,
            EscrowError::InvalidEscrowState
        );

        require!(
            amount == escrow_account.amount,
            EscrowError::InvalidDepositAmount
        );

        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(
                maintainer.to_account_info().key,
                escrow_account.to_account_info().key,
                amount,
            ),
            &[
                maintainer.to_account_info(),
                escrow_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        escrow_account.state = EscrowState::Funded;
        Ok(())
    }

    pub fn assign_contributor(ctx: Context<AssignContributor>, contributor: Pubkey) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;

        require!(
            escrow_account.state == EscrowState::Funded,
            EscrowError::InvalidEscrowState
        );

        escrow_account.contributor = contributor;
        escrow_account.state = EscrowState::Assigned;
        Ok(())
    }

    pub fn release_funds(ctx: Context<ReleaseFunds>) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        let contributor = &ctx.accounts.contributor;

        require!(
            escrow_account.state == EscrowState::Assigned,
            EscrowError::InvalidEscrowState
        );

        require!(
            contributor.key() == escrow_account.contributor,
            EscrowError::InvalidContributor
        );

        **escrow_account.to_account_info().try_borrow_mut_lamports()? -= escrow_account.amount;
        **contributor.to_account_info().try_borrow_mut_lamports()? += escrow_account.amount;

        escrow_account.state = EscrowState::Completed;
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

#[derive(Accounts)]
pub struct DepositFunds<'info> {
    #[account(mut)]
    pub maintainer: Signer<'info>,
    #[account(
        mut,
        constraint = escrow_account.maintainer == maintainer.key(),
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AssignContributor<'info> {
    #[account(mut)]
    pub maintainer: Signer<'info>,
    #[account(
        mut,
        constraint = escrow_account.maintainer == maintainer.key(),
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
}

#[derive(Accounts)]
pub struct ReleaseFunds<'info> {
    #[account(mut)]
    pub maintainer: Signer<'info>,
    #[account(mut)]
    /// CHECK: This is safe because we're checking the pubkey in the instruction
    pub contributor: AccountInfo<'info>,
    #[account(
        mut,
        constraint = escrow_account.maintainer == maintainer.key(),
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

#[error_code]
pub enum EscrowError {
    #[msg("The escrow is not in the correct state for this operation")]
    InvalidEscrowState,
    #[msg("The deposit amount does not match the bounty amount")]
    InvalidDepositAmount,
    #[msg("The contributor does not match the assigned contributor")]
    InvalidContributor,
}
