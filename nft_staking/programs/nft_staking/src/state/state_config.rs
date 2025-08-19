use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StateConfig {
    pub points_per_stake: u8,
    pub max_stake: u8,
    pub freeze_period: u32,
    pub rewards_period: u32,
    pub bump: u8,
}