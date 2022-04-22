use crate::schema::*;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};

#[derive(Accounts)]
pub struct InitializeProposal<'info> {
    // TODO: Customize account address
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = Proposal::SIZE,
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(seeds = [b"treasurer", &proposal.key().to_bytes()], bump)]
    /// CHECK: Just a pure account
    pub treasurer: AccountInfo<'info>,
    pub mint: Box<Account<'info, token::Mint>>,
    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = treasurer
      )]
    pub proposal_token_account: Account<'info, token::TokenAccount>,
    // System Program Address
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn exec(ctx: Context<InitializeProposal>, start_date: i64, end_date: i64) -> Result<()> {
    ctx.accounts.proposal.start_date = start_date;
    ctx.accounts.proposal.end_date = end_date;
    ctx.accounts.proposal.accepted_power = 0;
    ctx.accounts.proposal.rejected_power = 0;
    ctx.accounts.proposal.mint = ctx.accounts.mint.key();
    Ok(())
}
