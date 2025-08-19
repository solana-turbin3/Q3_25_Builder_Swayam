pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
use anchor_lang::prelude::*;
pub use constants::*;
pub use state::*;
pub use instructions::*;

declare_id!("GPtjsrW8gCZTbaMo7z1hx7YSanwo5AeecZpTWCZN4YDE");

#[program]
pub mod capstone_project {
    use super::*;

   pub fn create_task(
        ctx: Context<CreateTask>,
        task_seed: u64,
        reward_amount: u64,
        total_work_units: u64,
        task_uri: String,
    ) -> Result<()> {
        ctx.accounts.create_task(
            task_seed,
            reward_amount,
            total_work_units,
            task_uri,
            &ctx.bumps,
        )
    }

    pub fn submit_contribution(
        ctx: Context<SubmitContribution>,
        submission_uri: String,
        work_units: u64,
    ) -> Result<()> {
        ctx.accounts.submit_contribution(
            submission_uri,
            work_units,
            &ctx.bumps,
        )
    }

    pub fn reviewer_init(
        ctx: Context<ReviewerInit>,
        stake_amount: u64,
    ) -> Result<()> {
        ctx.accounts.reviewer_init(stake_amount, &ctx.bumps)
    }

    pub fn submit_vote(
        ctx: Context<SubmitVote>,
        approve: bool,
    ) -> Result<()> {
        ctx.accounts.submit_vote(approve)
    }

    pub fn finalize_contribution(
        ctx: Context<FinalizeContribution>,
        approval_threshold: u8,
    ) -> Result<()> {
        ctx.accounts.finalize_contribution(approval_threshold)
    }

}
