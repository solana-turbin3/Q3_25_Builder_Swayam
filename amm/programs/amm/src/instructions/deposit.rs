#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{transfer, Mint, TokenAccount, TokenInterface, Transfer , MintTo , mint_to}};
use constant_product_curve::ConstantProduct;
use crate::{error::AmmError, state::Config};
#[derive(Accounts)]
pub struct Deposit <'info> { 
    #[account(mut)]
    pub user : Signer<'info>,

    #[account(
        mut , 
        associated_token::mint = mint_x,
        associated_token::authority = user
    )]
    pub user_ata_x : InterfaceAccount<'info , TokenAccount>,

    #[account(
        mut , 
        associated_token::mint = mint_y,
        associated_token::authority = user
    )]
    pub user_ata_y : InterfaceAccount<'info , TokenAccount>,

    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint = mint_lp,
        associated_token::authority = user
    )]
    pub user_ata_lp : InterfaceAccount<'info , TokenAccount>,

    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"config" , config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config : Account<'info , Config>,

     #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config
    )]
    pub vault_x : InterfaceAccount<'info , TokenAccount >,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config
    )]
    pub vault_y : InterfaceAccount<'info , TokenAccount >,

    #[account(
        mut,
        seeds = [b"lp" , config.key().as_ref()],
        bump = config.lp_bump,
    )]
    pub mint_lp : InterfaceAccount<'info , Mint>,

    pub mint_x : InterfaceAccount<'info , Mint>,
    pub mint_y : InterfaceAccount<'info , Mint>,

    pub system_program : Program<'info , System>,
    pub associated_token_program : Program<'info , AssociatedToken>,
    pub token_program : Interface<'info , TokenInterface>

}
impl<'info> Deposit <'info> {

    pub fn deposit(&mut self , amount : u64 , max_x:u64 , max_y:u64 ) ->Result<()>{
        require!(self.config.locked == false , AmmError::PoolLocked);
        require!(amount!=0 , AmmError::InvalidAmount);

        let (x,y) = match self.mint_lp.supply == 0 && self.vault_x.amount == 0 && self.vault_y.amount == 0{
            true => (max_x,max_y),
            false => {
                let amount = ConstantProduct::xy_deposit_amounts_from_l(self.vault_x.amount, self.vault_y.amount, self.mint_lp.supply, amount, 6).unwrap();
                (amount.x , amount.y)
            }
        };
        require!(x <= max_x && y <= max_y , AmmError::SlippageExceeded);
        self.deposit_tokens(true, x)?;
        self.deposit_tokens(false, y)?;
        self.mint_lp_tokens(amount)
        
    }

    pub fn deposit_tokens(&self , is_x:bool , amount : u64) ->Result<()> {

        let (from , to ) = match is_x {
            true => (self.user_ata_x.to_account_info() , self.vault_x.to_account_info()), 
            false => (self.user_ata_y.to_account_info() , self.vault_y.to_account_info())
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer{
            from , 
            to, 
            authority : self.user.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        transfer(cpi_ctx, amount)
    }

    pub fn mint_lp_tokens(&self , amount : u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo{
            mint : self.mint_lp.to_account_info(),
            to : self.user_ata_lp.to_account_info(),
            authority: self.config.to_account_info()
        };

        let seeds: &[&[&[u8]]] = &[&[&b"config"[..], &self.config.seed.to_le_bytes(), &[self.config.config_bump]]];
        // let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);
        mint_to(cpi_ctx, amount)
    } 
}