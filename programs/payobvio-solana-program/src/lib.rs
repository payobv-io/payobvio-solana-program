use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

pub mod error;
pub mod states;
pub mod structs;
use crate::{error::*, states::*, structs::*};

declare_id!("FB7QEze2Xmxiw4oh1SfM2WyCwEgAY6oY27dbLPQ365HT");

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
        escrow_account.state = EscrowState::Initialized;
        Ok(())
    }

    pub fn close_escrow(ctx: Context<CloseEscrow>) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;

        // require!(
        //     escrow_account.state == EscrowState::Funded,
        //     EscrowError::InvalidEscrowState
        // );

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
    pub fn release_funds(ctx: Context<ReleaseFunds>) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        let contributor = &ctx.accounts.contributor;

        require!(
            escrow_account.state == EscrowState::Funded,
            EscrowError::InvalidEscrowState
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
            escrow_account.state == EscrowState::Funded,
            EscrowError::InvalidEscrowState
        );

        **escrow_account.to_account_info().try_borrow_mut_lamports()? -= escrow_account.amount;
        **maintainer.to_account_info().try_borrow_mut_lamports()? += escrow_account.amount;

        escrow_account.state = EscrowState::Refunded;
        Ok(())
    }
}
