use anchor_lang::prelude::*;

declare_id!("1RY8PMr58YJVmHByVptUgWXfvDpRfCjEem5UBTsQYdB");

#[program]
pub mod blueshift_anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
