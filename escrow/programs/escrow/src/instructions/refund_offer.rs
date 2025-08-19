use anchor_lang::prelude::*;
use crate::Offer;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{transfer_checked , Mint, TokenAccount , TokenInterface ,TransferChecked ,CloseAccount, close_account}
}; 

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Refund<'info> {
    #[account(
        mut 
    )]
    pub maker : Signer<'info>,

    #[account(
       mint::token_program = token_program
    )]
    pub token_mint_a : InterfaceAccount<'info , Mint>,

    #[account(
        mut ,
        associated_token::token_program = token_program ,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker
    )]
    pub maker_ata_a : InterfaceAccount<'info , TokenAccount>,
    #[account(
        mut , 
        close = maker ,
        has_one = token_mint_a,
        has_one = maker,
        seeds = [b"escrow" , maker.key.as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump , 
    )]
    pub escrow : Account<'info , Offer>,

     #[account(
        associated_token::mint = token_mint_a,
        associated_token::token_program = token_program,
        associated_token::authority = escrow,
    )]
    pub vault : InterfaceAccount<'info , TokenAccount>,

    pub token_program : Interface<'info , TokenInterface>,
    pub associated_token_program : Program<'info , AssociatedToken>,
    pub system_program : Program<'info , System>
}

impl<'info> Refund<'info> {
    pub fn refund_and_close_vault(&mut self) -> Result<()>{
        let maker_key = self.maker.to_account_info().key();
        let signer_seeds: [&[&[u8]]; 1] =[&[b"escrow" , 
        maker_key.as_ref(), 
        &self.escrow.seed.to_be_bytes()[..], 
        &[self.escrow.bump], 
        ]];

        let transfer_accounts = TransferChecked{
            from : self.vault.to_account_info(),
            mint : self.token_mint_a.to_account_info(),
            to : self.maker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info()
        };
        let transfer_cpi_context = CpiContext::new_with_signer(self.token_program.to_account_info(), transfer_accounts, &signer_seeds);
        transfer_checked(transfer_cpi_context, self.vault.amount, self.token_mint_a.decimals)?;
        let close_accounts = CloseAccount{
            account : self.vault.to_account_info(),
            destination : self.maker.to_account_info(),
            authority : self.escrow.to_account_info()
        };
        let close_cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), close_accounts, &signer_seeds);
        close_account(close_cpi_ctx)?;
        Ok(())
    }
}