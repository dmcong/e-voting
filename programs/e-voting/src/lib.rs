use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod instructions;
pub use instructions::*;

pub mod schema;
pub use schema::*;

pub mod utils;
pub use utils::*;

pub mod errors;
pub use errors::*;

#[program]
pub mod e_voting {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeProposal>,
        start_date: i64,
        end_date: i64,
    ) -> Result<()> {
        initialize_proposal::exec(ctx, start_date, end_date)
    }

    pub fn vote(ctx: Context<Vote>, accepted_amount: u64, rejected_amount: u64) -> Result<()> {
        vote::exec(ctx, accepted_amount, rejected_amount)
    }

    pub fn close(ctx: Context<Close>, accepted_amount: u64, rejected_amount: u64) -> Result<()> {
        close::exec(ctx, accepted_amount, rejected_amount)
    }
}
