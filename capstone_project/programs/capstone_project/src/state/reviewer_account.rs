use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ReviewAccount {
    pub staked_amount: u64,
    pub active: bool,
    pub created_at: i64,         // Timestamp
    pub state_bump: u8,
    pub reviewer_vault_bump: u8, // PDA bump
}
