use anchor_lang::prelude::*;

use crate::states::EscrowState;

#[derive(Accounts)]
#[instruction(bounty_amount: u64, issue_id: String)]
pub struct InitializeEscrow<'info> {
    #[account(mut)]
    pub maintainer: Signer<'info>,
    #[account(
        init,
        payer = maintainer,
        space = 8 + 32 + 8 + 32 + 1 + issue_id.len(),
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
pub struct ReleaseFunds<'info> {
    #[account(mut)]
    pub maintainer: Signer<'info>,
    #[account(mut)]
    /// CHECK: This account is not read or written in the instruction, just receives funds
    pub contributor: AccountInfo<'info>,
    #[account(
        mut,
        constraint = escrow_account.maintainer == maintainer.key(),
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maintainer: Signer<'info>,
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
    pub amount: u64,
    pub issue_id: String,
    pub state: EscrowState,
}
