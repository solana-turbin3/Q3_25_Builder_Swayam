pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CS2zG8rTZ78jpsaaoXhr9scAudp8bz6vjkbsia5tJTne");

#[program]
pub mod amm {
    
}
