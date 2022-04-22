use anchor_lang::prelude::*;

#[account]
pub struct Receipt {
  pub authority: Pubkey,
  pub proposal: Pubkey,
  pub accepted_power: u64,
  pub rejected_power: u64,
}

impl Receipt {
  pub const SIZE: usize = 8 + 32 + 32 + 8 + 8;
}
