pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6YMmPCizv2jPS74brRFceaCV4FaW6vRpWVCq4rfvyGn");

#[program]
pub mod nft_staking {
    use super::*;

    
}
