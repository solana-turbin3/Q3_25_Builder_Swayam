use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;
#[derive(Accounts)]
pub struct SubmitVote<'info> {
    #[account(mut)]
    pub reviewer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"reviewer", reviewer.key().as_ref()],
        bump = review_account.state_bump,
        constraint = review_account.active @ ErrorCode::ReviewerNotActive,
        constraint = review_account.staked_amount > 0 @ ErrorCode::NoStake
    )]
    pub review_account: Account<'info, ReviewAccount>,

    // The contribution being voted on
    #[account(
        seeds = [b"contribution", task_account.key().as_ref(), contribution_account.contributor.as_ref()],
        bump = contribution_account.bump
    )]
    pub contribution_account: Account<'info, ContributionAccount>,

     #[account(
        seeds = [b"task", task_account.creator.as_ref(), task_account.task_seed.to_le_bytes().as_ref()],
        bump = task_account.task_bump
    )]
    pub task_account: Account<'info, TaskAccount>,

      #[account(
        mut,
        seeds = [b"vote", contribution_account.key().as_ref()],
        bump = vote_account.bump
    )]
    pub vote_account: Account<'info, VoteAccount>,

    pub system_program: Program<'info, System>,
}
impl<'info> SubmitVote<'info> {
    pub fn submit_vote(
        &mut self,
        approve: bool, // true = approve, false = reject
    ) -> Result<()> {
        // Prevent double-voting
        if self.vote_account.voters.contains(&self.reviewer.key()) {
            return Err(error!(ErrorCode::AlreadyVoted));
        }

        // Record vote
        self.vote_account.total_votes += 1;
        if approve {
            self.vote_account.approve_votes += 1;
        }
        self.vote_account.voters.push(self.reviewer.key());

        Ok(())
    }
}
