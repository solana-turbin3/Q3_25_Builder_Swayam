use anchor_lang::prelude::*;
use crate::{state::*};
use crate::error::ErrorCode::InvalidWorkUnits;
#[derive(Accounts)]
pub struct SubmitContribution<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(
        mut,
        seeds = [b"task", task_account.creator.as_ref(), task_account.task_seed.to_le_bytes().as_ref()],
        bump = task_account.task_bump,
    )]
    pub task_account: Account<'info, TaskAccount>,

    #[account(
        init,
        payer = contributor,
        seeds = [b"contribution", task_account.key().as_ref(), contributor.key().as_ref()],
        bump,
        space = 8 + ContributionAccount::INIT_SPACE
    )]
    pub contribution_account: Account<'info, ContributionAccount>,

    #[account(
        init ,
        payer = contributor,
        seeds = [b"vote", contribution_account.key().as_ref()],
        bump,
        space = 8 + VoteAccount::INIT_SPACE
    )]
    pub vote_account: Account<'info, VoteAccount>,

    pub system_program: Program<'info, System>,
}
impl<'info> SubmitContribution<'info> {
    pub fn submit_contribution(
        &mut self,
        submission_uri: String,
        work_units: u64,
        bumps: &SubmitContributionBumps,
    ) -> Result<()> {
        if work_units == 0 || work_units > self.task_account.total_work_units {
            return Err(error!(InvalidWorkUnits));
        }
        self.contribution_account.set_inner(ContributionAccount{
            contributor: self.contributor.key(),
            submission_uri,
            work_units,
            approved: false,
            created_at: Clock::get()?.unix_timestamp,
            bump: bumps.contribution_account,
        });
        self.vote_account.set_inner(VoteAccount {
            total_votes: 0,
            approve_votes: 0,
            voters: vec![],
            bump: bumps.vote_account,
        });
        self.task_account.total_submissions += 1;
        self.task_account.updated_at = Clock::get()?.unix_timestamp;

        Ok(())
    }
}