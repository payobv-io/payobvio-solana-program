use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("The escrow is not in the correct state for this operation")]
    InvalidEscrowState,
    #[msg("The deposit amount does not match the bounty amount")]
    InvalidDepositAmount,
}
