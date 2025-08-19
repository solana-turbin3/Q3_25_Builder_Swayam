use anchor_lang::prelude::*;
use crate::{state::*};
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct ReviewerInit<'info> {
    #[account(mut)]
    pub reviewer: Signer<'info>,

    #[account(
        init,
        payer = reviewer,
        seeds = [b"reviewer", reviewer.key().as_ref()],
        bump,
        space = 8 + ReviewAccount::INIT_SPACE
    )]
    pub review_account: Account<'info, ReviewAccount>,

    // Reviewer vault PDA (SystemAccount) to store staked SOL
    #[account(
        mut,
        seeds = [b"reviewer_vault", reviewer.key().as_ref()],
        bump
    )]
    pub reviewer_stake_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}
impl <'info> ReviewerInit<'info> {
    pub fn reviewer_init(&mut self, staked_amount: u64, bumps: &ReviewerInitBumps) -> Result<()> {
        self.review_account.set_inner(ReviewAccount {
            staked_amount,
            active: true,
            created_at: Clock::get()?.unix_timestamp,
            state_bump: bumps.review_account,
            reviewer_vault_bump: bumps.reviewer_stake_vault
        });

        self.deposit_stake(staked_amount)?;

        Ok(())
    }

    pub fn deposit_stake(&mut self, staked_amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.reviewer.to_account_info(),
            to: self.reviewer_stake_vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, staked_amount)?;
        Ok(())
    }
    
}