use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("The proposal isn't active")]
  NotActiveProposal,
  #[msg("The proposal is active")]
  ActiveProposal,
  #[msg("Invalid accepted amount")]
  InvalidAcceptedAmount,
  #[msg("Invalid rejected amount")]
  InvalidRejectedAmount,
}
