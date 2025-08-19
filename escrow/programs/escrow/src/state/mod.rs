use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub seed : u64, 
    pub maker : Pubkey , 
    pub token_mint_a : Pubkey ,
    // token wanted 
    pub token_mint_b : Pubkey, 
    pub token_b_amount_wanted : u64 , 
    // used to calculate address for this account, storing here for performance optimization 
    pub bump : u8
}

