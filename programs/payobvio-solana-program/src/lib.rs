use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

pub mod error;
pub mod states;
pub mod structs;
use crate::{error::*, states::*, structs::*};

declare_id!("Bo9Nd6FuheXnZoPx19s4uWwSjuLct3nHnZ7ZjrKdSUGS");

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

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        let maintainer = &ctx.accounts.maintainer;

        require!(
            escrow_account.state == EscrowState::Funded
                || escrow_account.state == EscrowState::Assigned,
            EscrowError::InvalidEscrowState
        );

        **escrow_account.to_account_info().try_borrow_mut_lamports()? -= escrow_account.amount;
        **maintainer.to_account_info().try_borrow_mut_lamports()? += escrow_account.amount;

        escrow_account.state = EscrowState::Refunded;
        Ok(())
    }
}
