use anchor_lang::prelude::*;

declare_id!("CxL91vmcDCWVFY84BvY1TaU8hwoRutgzCgrjGEQ3hhYm");

#[program]
pub mod curs_7 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
