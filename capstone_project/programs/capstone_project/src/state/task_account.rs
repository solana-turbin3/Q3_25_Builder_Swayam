use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TaskAccount {
    pub creator: Pubkey,
    pub reward_amount: u64,
    pub total_work_units: u64,
    #[max_len(200)]
    pub task_uri : String,
    pub status: TaskStatus,
    pub total_submissions: u32,
    pub created_at: i64,
    pub updated_at: i64,
    pub task_seed : u64, // Unique seed for the task
    pub task_bump: u8,
    pub vault_bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq , InitSpace)]
pub enum TaskStatus {
    Created,
    SubmissionsOpen,
    UnderReview,
    Finalized,
}