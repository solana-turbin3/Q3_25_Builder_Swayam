use anchor_lang::prelude::{*};
use anchor_spl::{associated_token::AssociatedToken, token_interface::{transfer_checked , Mint, TokenAccount , TokenInterface ,TransferChecked}
}; 
use crate::Offer;

pub fn make_offer(_context : Context<MakeOffer>) -> Result<()> {

    Ok(())
}


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct MakeOffer<'info>{
    #[account(mut)]
    pub maker : Signer<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_a : InterfaceAccount<'info , Mint>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_b : InterfaceAccount<'info , Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a : InterfaceAccount<'info , TokenAccount>,

    #[account(
        init , 
        payer = maker ,
        associated_token::mint = mint_a,
        associated_token::token_program = token_program,
        associated_token::authority = escrow,
    )]
    pub vault : InterfaceAccount<'info , TokenAccount>,

    #[account(
        init , 
        payer = maker , 
        seeds = [b"escrow" , maker.key.as_ref(), seed.to_le_bytes().as_ref()],
        bump , 
        space = Offer::INIT_SPACE + 8 
    )]
    pub escrow : Account<'info , Offer>,

    pub token_program : Interface<'info , TokenInterface>,
    pub associated_token_program : Program<'info , AssociatedToken>,
    pub system_program : Program<'info , System>
}

impl<'info> MakeOffer<'info> {
    pub fn init_escrow(&mut self , seed : u64 , receive : u64 , bumps : &MakeOfferBumps)-> Result<()> {
        self.escrow.set_inner(Offer { seed, 
        maker: self.maker.key() , 
        token_mint_a: self.mint_a.key(), 
        token_mint_b: self.mint_b.key(), 
        token_b_amount_wanted: receive,
        bump: bumps.escrow 
    });
        Ok(())    
    }

    pub fn deposit(&mut self , deposit : u64) -> Result<()> {
        let transfer_accounts = TransferChecked{
            from : self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority:self.maker.to_account_info()
        };
        let cpi_context = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);
        transfer_checked(cpi_context, deposit, self.mint_a.decimals)
    }
}