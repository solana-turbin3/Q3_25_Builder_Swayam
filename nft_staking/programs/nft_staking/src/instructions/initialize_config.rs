use anchor_lang::prelude::*;
use anchor_spl::token::{Mint , Token};
use crate::state_config::StateConfig;


#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin : Signer<'info>,

    #[account(
        init ,
        payer = admin ,
        space = 8 + StateConfig::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config : Account<'info , StateConfig>,

    #[account(
        init_if_needed , 
        payer = admin , 
        seeds = [b"rewards_mint",  config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config
    )]
    pub rewards_mint : Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
impl<'info> InitializeConfig<'info> {
    pub fn initialize_config(&mut self,  points_per_stake:u8 , max_stake : u8 , rewards_period: u32 , freeze_period:u32, bumps : &InitializeConfigBumps ) -> Result<()> {
        self.config.set_inner(StateConfig { 
            points_per_stake, 
            max_stake,
            freeze_period,
            rewards_period,
            bump: bumps.config 
        });
        Ok(())
    }
}
pub fn handler(ctx: Context<InitializeConfig>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}
