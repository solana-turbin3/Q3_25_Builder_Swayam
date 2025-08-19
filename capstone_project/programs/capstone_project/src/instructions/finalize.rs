use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct FinalizeContribution<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // Task creator or PDA authority

    #[account(
        mut,
        seeds = [b"task", task_account.creator.as_ref(), task_account.task_seed.to_le_bytes().as_ref()],
        bump = task_account.task_bump,
        constraint = task_account.status != TaskStatus::Finalized @ ErrorCode::TaskAlreadyFinalized,
        constraint = authority.key() == task_account.creator @ ErrorCode::Unauthorized

    )]
    pub task_account: Account<'info, TaskAccount>,

    #[account(
        mut,
        seeds = [b"contribution", task_account.key().as_ref(), contribution_account.contributor.as_ref()],
        bump = contribution_account.bump,
        constraint = !contribution_account.approved @ ErrorCode::ContributionAlreadyFinalized
    )]
    pub contribution_account: Account<'info, ContributionAccount>,

    #[account(
        seeds = [b"vote", contribution_account.key().as_ref()],
        bump = vote_account.bump
    )]
    pub vote_account: Account<'info, VoteAccount>,

    /// CHECK: Vault holding escrowed SOL
    #[account(
        mut,
        seeds = [b"escrow", task_account.key().as_ref()],
        bump = task_account.vault_bump
    )]
    pub escrow_vault: SystemAccount<'info>,

    /// CHECK: Contributor to be paid
    #[account(mut)]
    pub contributor: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> FinalizeContribution<'info> {
    pub fn finalize_contribution(&mut self, approval_threshold: u8) -> Result<()> {
        // Avoid division rounding issues
        require!(self.vote_account.total_votes > 0, ErrorCode::NoVotesCast);

        let approval_percent =
            (self.vote_account.approve_votes * 100) / self.vote_account.total_votes;

        if approval_percent >= approval_threshold.into() {
            // Calculate reward for this contribution
            let reward_per_unit =
                self.task_account.reward_amount / self.task_account.total_work_units;
            let payout = reward_per_unit * self.contribution_account.work_units;

            // Transfer payout from escrow to contributor
            let task_key = self.task_account.key();
            let seeds: &[&[u8]] = &[
                          b"escrow",
                          task_key.as_ref(),
                        &[self.task_account.vault_bump],
            ];
            let signer_seeds = &[&seeds[..]];

            let cpi_accounts = Transfer {
                from: self.escrow_vault.to_account_info(),
                to: self.contributor.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                cpi_accounts,
                signer_seeds,
            );

            transfer(cpi_ctx, payout)?;
            self.contribution_account.approved = true;
        } else {
            // Rejected — no payout
            self.contribution_account.approved = false;
        }

        Ok(())
    }
}