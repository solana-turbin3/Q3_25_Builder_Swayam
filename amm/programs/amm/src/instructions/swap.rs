#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::error::AmmError;
use crate::state::Config;

#[derive(Accounts)]
pub struct Swap<'info> {
    
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub mint_x: Account<'info, Mint>,
   
    #[account(mint::token_program = token_program)]
    pub mint_y: Account<'info, Mint>,

    #[account(
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
        has_one = mint_x,  // Ensures mint_x matches the one in config
        has_one = mint_y,  // Ensures mint_y matches the one in config
    )]
    pub config: Account<'info, Config>,

   
    #[account(
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump
    )]
    pub mint_lp: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    pub vault_y: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_x: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_y: Account<'info, TokenAccount>,

    /// SPL Token program for token operations
    pub token_program: Program<'info, Token>,
    /// Associated Token program for ATA operations
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// System program for account creation
    pub system_program: Program<'info, System>,
}

impl<'info> Swap<'info> {
   
    pub fn swap(&mut self, is_x: bool, amount_in: u64, min_amount_out: u64) -> Result<()> {
        // Ensure the pool is not locked 
        require!(!self.config.locked, AmmError::PoolLocked);
        // Ensure user is swapping a positive amount
        require!(amount_in > 0, AmmError::InvalidAmount);

        // Initialize constant product curve with current pool state
        let mut curve = ConstantProduct::init(
            self.vault_x.amount,    
            self.vault_y.amount,    
            self.mint_lp.supply,    
            self.config.fee,        
            None,                  
        )
        .map_err(AmmError::from)?;

        // Determine which token is being swapped in
        let p = match is_x {
            true => LiquidityPair::X,   
            false => LiquidityPair::Y,  
        };

        // Calculate swap amounts using constant product formula
        let swap_result = curve
            .swap(p, amount_in, min_amount_out)
            .map_err(AmmError::from)?;
        
        require!(swap_result.deposit != 0, AmmError::InvalidAmount);
        require!(swap_result.withdraw != 0, AmmError::InvalidAmount);

        
        self.deposit_token(is_x, swap_result.deposit)?;      
        self.withdraw_token(!is_x, swap_result.withdraw)?;   

        Ok(())
    }

    
    pub fn deposit_token(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true => (
                self.user_ata_x.to_account_info(),    
                self.vault_x.to_account_info(),       
                self.mint_x.to_account_info(),        
                self.mint_x.decimals,                 
            ),
            false => (
                self.user_ata_y.to_account_info(),    
                self.vault_y.to_account_info(),       
                self.mint_y.to_account_info(),        
                self.mint_y.decimals,                 
            ),
        };

        let cpi_program = self.token_program.to_account_info();

        // Set up transfer instruction accounts
        let cpi_accounts = TransferChecked {
            from,
            to,
            authority: self.user.to_account_info(),
            mint,
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        
        transfer_checked(cpi_context, amount, decimals)
    }

    pub fn withdraw_token(&mut self, is_x: bool, amount: u64) -> Result<()> {
        // Select appropriate accounts based on token type
        let (from, to, mint, decimals) = match is_x {
            true => (
                self.vault_x.to_account_info(),       // Transfer from vault X
                self.user_ata_x.to_account_info(),    // Transfer to user's X account
                self.mint_x.to_account_info(),        // Token X mint
                self.mint_x.decimals,                 // Token X decimals
            ),
            false => (
                self.vault_y.to_account_info(),       // Transfer from vault Y
                self.user_ata_y.to_account_info(),    // Transfer to user's Y account
                self.mint_y.to_account_info(),        // Token Y mint
                self.mint_y.decimals,                 // Token Y decimals
            ),
        };

        let cpi_program = self.token_program.to_account_info();

        // Set up transfer instruction accounts
        let cpi_accounts = TransferChecked {
            from,
            to,
            mint,
            authority: self.config.to_account_info(),  // Config PDA signs the transfer
        };

        // Create signer seeds for config PDA
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"config",
            &self.config.seed.to_le_bytes(),
            &[self.config.config_bump],
        ]];

        // Create CPI context with PDA signer
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Execute the transfer with amount and decimal validation
        transfer_checked(cpi_context, amount, decimals)
    }
}