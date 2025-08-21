#[derive(Accounts)]
pub struct Refund<'info> {
  #[account(mut)]
  pub Refundr: Signer<'info>,
  #[account(mut)]
  pub maker: SystemAccount<'info>,
  #[account(
      mut,
      close = maker,
      seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
      bump = escrow.bump,
      has_one = maker @ EscrowError::InvalidMaker,
      has_one = mint_a @ EscrowError::InvalidMintA,
      has_one = mint_b @ EscrowError::InvalidMintB,
  )]
  pub escrow: Box<Account<'info, Escrow>>,
 
  /// Token Accounts
  pub mint_a: Box<InterfaceAccount<'info, Mint>>,
  pub mint_b: Box<InterfaceAccount<'info, Mint>>,
  #[account(
      mut,
      associated_token::mint = mint_a,
      associated_token::authority = escrow,
      associated_token::token_program = token_program
  )]
  pub vault: Box<InterfaceAccount<'info, TokenAccount>>,
  #[account(
      init_if_needed,
      payer = Refundr,
      associated_token::mint = mint_a,
      associated_token::authority = Refundr,
      associated_token::token_program = token_program
  )]
  pub maker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,
  #[account(
      mut,
      associated_token::mint = mint_b,
      associated_token::authority = Refundr,
      associated_token::token_program = token_program
  )]
  /// Programs
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_program: Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>,
}

impl<'info> Refund<'info> {
    fn transfer_to_maker(&mut self) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.Refundr_ata_b.to_account_info(),
                    to: self.maker_ata_b.to_account_info(),
                    mint: self.mint_b.to_account_info(),
                    authority: self.Refundr.to_account_info(),
                },
            ),
            self.escrow.receive,
            self.mint_b.decimals,
        )?;
 
        Ok(())
    }
 
    fn withdraw_and_close_vault(&mut self) -> Result<()> {
        // Create the signer seeds for the Vault
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];
 
        // Transfer Token A (Vault -> Refundr)
        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.vault.to_account_info(),
                    to: self.Refundr_ata_a.to_account_info(),
                    mint: self.mint_a.to_account_info(),
                    authority: self.escrow.to_account_info(),
                },
                &signer_seeds,
            ),
            self.vault.amount,
            self.mint_a.decimals,
        )?;
 
        // Close the Vault
        close_account(CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            CloseAccount {
                account: self.vault.to_account_info(),
                authority: self.escrow.to_account_info(),
                destination: self.maker.to_account_info(),
            },
            &signer_seeds,
        ))?;
 
        Ok(())
    }
}

pub fn handler(ctx: Context<Refund>) -> Result<()> {
    // Transfer Token B to Maker
    ctx.accounts.transfer_to_maker()?;
 
    // Withdraw and close the Vault
    ctx.accounts.withdraw_and_close_vault()?;
 
    Ok(())
}
