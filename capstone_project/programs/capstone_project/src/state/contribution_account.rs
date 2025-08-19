use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ContributionAccount {
    pub contributor: Pubkey,
    #[max_len(200)]
    pub submission_uri : String,
    pub work_units: u64,
    pub approved : bool,
    pub created_at: i64,
    pub bump: u8,
}