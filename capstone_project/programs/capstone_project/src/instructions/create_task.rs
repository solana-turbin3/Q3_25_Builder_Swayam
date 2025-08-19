use anchor_lang::prelude::*;
use crate::{state::*};
use anchor_lang::system_program::{transfer};
use anchor_lang::system_program::{Transfer};

#[derive(Accounts)]
#[instruction(task_seed: u64)]
pub struct CreateTask<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    
    #[account(
        init,
        payer = creator,
        seeds = [b"task", creator.key().as_ref(), task_seed.to_le_bytes().as_ref()],
        bump,
        space = 8 + TaskAccount::INIT_SPACE
    )]
    pub task_account: Account<'info, TaskAccount>,

    #[account(
        mut,
        seeds = [b"escrow", task_account.key().as_ref()],
        bump
    )]
    pub escrow_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl <'info> CreateTask<'info> {
   pub fn create_task( &mut self, task_seed:u64,  reward_amount: u64, total_work_units: u64, task_uri: String, bumps : &CreateTaskBumps) -> Result<()> {
        self.task_account.set_inner(TaskAccount {
            creator : self.creator.key(),
            reward_amount,
            total_work_units,
            task_uri,
            status: TaskStatus::Created,
            total_submissions: 0,
            created_at: Clock::get()?.unix_timestamp,
            updated_at: Clock::get()?.unix_timestamp,
            task_seed: task_seed,
            task_bump : bumps.task_account,
            vault_bump: bumps.escrow_vault,
        });       
        self.deposit_reward(reward_amount)?;

        Ok(())
    }

   pub fn deposit_reward(&mut self , reward_amount: u64) -> Result<()> {
       let cpi_program = self.system_program.to_account_info();
       let cpi_accounts  = Transfer {
           from: self.creator.to_account_info(),
           to: self.escrow_vault.to_account_info(),
       };
       let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
       transfer(cpi_ctx, reward_amount)?;
       Ok(())
   }
}