use anchor_lang::prelude::*;
// This Vote Account stores votes for a single contribution made by a worker. 
// It is used to track all the votes made by reviewers on whether to approve or reject the contribution.
#[account]
#[derive(InitSpace)]
pub struct VoteAccount {
    pub total_votes: u64,
    pub approve_votes: u64,
    #[max_len(50)]
    pub voters: Vec<Pubkey>, // Array of voter public keys
    pub bump: u8,
}
