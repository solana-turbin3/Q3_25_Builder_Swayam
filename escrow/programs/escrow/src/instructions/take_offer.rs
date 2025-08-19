use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken , 
    token_interface::{Mint, TokenAccount, TokenInterface , TransferChecked , transfer_checked , close_account , CloseAccount}};

use crate::Offer;

#[derive(Accounts)]
pub struct Take<'info> { 
    #[account(mut)]
    pub taker : Signer<'info>,
    #[account(mut)]
    pub maker : SystemAccount<'info>,

    pub token_mint_a : InterfaceAccount<'info , Mint>,
    pub token_mint_b : InterfaceAccount<'info , Mint>,

    #[account(
        init_if_needed, 
        payer = taker,
        associated_token::mint = token_mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_a : InterfaceAccount<'info , TokenAccount>,
    #[account(
        mut, 
        associated_token::mint = token_mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b : InterfaceAccount<'info , TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker ,
        associated_token::mint = token_mint_b , 
        associated_token::authority = maker
    )]
    pub maker_ata_b : InterfaceAccount<'info , TokenAccount>,

    #[account(
        mut , 
        associated_token::mint = token_mint_a , 
        associated_token::authority = escrow , 
        associated_token::token_program = token_program
    )]
    pub vault : InterfaceAccount<'info , TokenAccount>,
    #[account(
        mut , 
        close = maker ,
        has_one = maker , 
        has_one = token_mint_a,
        has_one = token_mint_b,
        seeds = [b"escrow" , maker.key().as_ref(), escrow.seed.to_be_bytes().as_ref()],
        bump = escrow.bump

    )]
    pub escrow : Account<'info , Offer>,

    pub token_program : Interface<'info , TokenInterface>,
    pub system_program : Program<'info , System>,
    pub associated_token_program : Program<'info , AssociatedToken>

}

impl<'info> Take<'info> {
    pub fn deposit(&mut self ) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked{
            from : self.taker_ata_b.to_account_info() ,
            mint : self.token_mint_b.to_account_info(),
            to : self.maker_ata_b.to_account_info(),
            authority : self.taker.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, self.escrow.token_b_amount_wanted, self.token_mint_b.decimals)?;
        Ok(())
    }
}