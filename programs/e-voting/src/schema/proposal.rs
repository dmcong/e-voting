use crate::utils::*;
use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub accepted_power: u64,
    pub rejected_power: u64,
    pub start_date: i64,
    pub end_date: i64,
}

impl Proposal {
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 8 + 8 + 8;

    pub fn is_active(&self) -> bool {
        let now = current_timestamp().unwrap_or(0);
        if now < self.start_date {
            return false;
        }
        if now > self.end_date {
            return false;
        }
        return true;
    }
}
