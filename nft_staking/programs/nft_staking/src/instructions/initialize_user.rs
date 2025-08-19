use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeUser <'info > { 
    #[account(mut)]
    pub user : Signer<'info>
}