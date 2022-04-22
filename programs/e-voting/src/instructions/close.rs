use crate::errors::ErrorCode;
use crate::schema::*;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};

#[derive(Accounts)]
pub struct Close<'info> {
    // TODO: Customize account address
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, has_one = mint)]
    // Proposal accounts
    pub proposal: Account<'info, Proposal>,
    #[account(seeds = [b"treasurer", &proposal.key().to_bytes()], bump)]
    /// CHECK: Just a pure account
    pub treasurer: AccountInfo<'info>,
    pub mint: Box<Account<'info, token::Mint>>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = treasurer
      )]
    pub proposal_token_account: Account<'info, token::TokenAccount>,
    // Wallet accounts
    #[account(
        seeds = [b"receipt".as_ref(), &proposal.key().to_bytes(), &authority.key().to_bytes()], 
        bump
    )]
    pub receipt: Account<'info, Receipt>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = authority
      )]
    pub wallet_token_account: Account<'info, token::TokenAccount>,
    // System Program Address
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn exec(ctx: Context<Close>, accepted_amount: u64, rejected_amount: u64) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let receipt = &mut ctx.accounts.receipt;
    if proposal.is_active() {
        return err!(ErrorCode::ActiveProposal);
    }
    if accepted_amount > receipt.accepted_power {
        return err!(ErrorCode::InvalidAcceptedAmount);
    }
    if rejected_amount > receipt.rejected_power {
        return err!(ErrorCode::InvalidRejectedAmount);
    }

    let seeds: &[&[&[u8]]] = &[&[
        "treasurer".as_ref(),
        &proposal.key().to_bytes(),
        &[*ctx.bumps.get("treasurer").unwrap()],
    ]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.proposal_token_account.to_account_info(),
            to: ctx.accounts.wallet_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
        seeds,
    );
    token::transfer(transfer_ctx, accepted_amount + rejected_amount)?;

    proposal.accepted_power -= accepted_amount;
    proposal.accepted_power -= rejected_amount;

    receipt.accepted_power -= accepted_amount;
    receipt.accepted_power -= rejected_amount;

    Ok(())
}
